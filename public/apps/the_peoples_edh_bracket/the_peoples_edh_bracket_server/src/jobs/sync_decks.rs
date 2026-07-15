use std::{collections::HashMap, num::NonZeroUsize};

use lazy_regex::regex::regex;
use sqlx::QueryBuilder;
use tracing::instrument;

use crate::{
    model::tracked_deck::{DeckMaindeckEntry, TrackedDeck, TrackedDeckCardType},
    state::AppState,
    util::{find_cards_by_names, find_cards_by_names_with_alternate_name},
};

#[instrument(skip(state))]
pub async fn sync_last_synced_deck(state: AppState) -> anyhow::Result<()> {
    tracing::info!("Starting tracked deck sync");

    let stalest_deck = sqlx::query_as!(
        TrackedDeck,
        "SELECT *
        FROM tracked_deck
        WHERE url_source IS NOT NULL
        ORDER BY last_synced ASC
        LIMIT 1"
    )
    .fetch_one(&state.pg_pool)
    .await
    .ok();

    let Some(deck) = stalest_deck else {
        tracing::info!("No tracked deck(s) found to sync");
        return Ok(());
    };

    match sync_deck(&state, &deck).await {
        Ok(()) => tracing::info!("Synced deck {}", deck.uuid),
        Err(err) => tracing::error!("Failed to sync deck {}: {}", deck.uuid, err),
    }

    sqlx::query!(
        "UPDATE tracked_deck
        SET last_synced = NOW()
        WHERE uuid = $1",
        deck.uuid
    )
    .execute(&state.pg_pool)
    .await?;

    Ok(())
}

#[instrument(skip(state))]
pub async fn sync_deck(state: &AppState, tracked_deck: &TrackedDeck) -> anyhow::Result<()> {
    tracing::info!("Syncing deck: {}", tracked_deck.uuid);
    let Some(url) = &tracked_deck.url_source else {
        tracing::warn!("Deck did not have a url source!");
        return Ok(());
    };

    let Some(deck_id) = regex!(r"^https?://(?:www\.)?moxfield\.com/decks/([A-Za-z0-9_-]+)(?:[/?#].*)?$")
        .captures(&url)
        .and_then(|captures| captures.get(1))
        .map(|deck_id| deck_id.as_str().to_string())
    else {
        tracing::warn!("Could not extract moxfield id from deck url: {}", url);
        return Ok(());
    };

    let moxfield_deck = state.moxfield_client.deck(&deck_id).await?;
    let commanders_cards = moxfield_deck.boards.commanders.cards;
    let mainboard_cards = moxfield_deck.boards.mainboard.cards;

    let (valid_commanders, invalid_commanders) =
        find_cards_by_names(commanders_cards.values().map(|entry| &entry.card.name), &state.pg_pool).await?;

    let card_names = mainboard_cards
        .values()
        .map(|entry| entry.card.name.clone())
        .collect::<Vec<_>>();
    let (valid_cards, invalid_card_names) = find_cards_by_names_with_alternate_name(&card_names, &state.pg_pool).await?;
    let card_counts = mainboard_cards
        .values()
        .map(|entry| {
            (
                entry.card.name.to_lowercase(),
                NonZeroUsize::try_from(entry.quantity).unwrap_or(NonZeroUsize::MIN),
            )
        })
        .collect::<HashMap<_, _>>();

    if !invalid_commanders.is_empty() || !invalid_card_names.is_empty() {
        return Err(anyhow::anyhow!(
            "Invalid cards: {:?} / {:?}",
            invalid_commanders,
            invalid_card_names
        ));
    }

    let valid_cards = valid_cards
        .into_iter()
        .map(|(card, alternate_name)| DeckMaindeckEntry {
            count: card_counts
                .get(&card.card.name.to_lowercase())
                .or_else(|| alternate_name.as_ref().and_then(|name| card_counts.get(&name.to_lowercase())))
                .cloned()
                .unwrap_or(NonZeroUsize::MIN),
            card,
        })
        .collect::<Vec<_>>();

    let mut tx = state.pg_pool.begin().await?;
    sqlx::query!(
        "DELETE FROM tracked_deck_card
        WHERE tracked_deck_uuid = $1",
        &tracked_deck.uuid
    )
    .execute(&mut *tx)
    .await?;

    let mut qb = QueryBuilder::new(
        "INSERT INTO tracked_deck_card (
			tracked_deck_uuid,
			ty,
			count,
			card_oracle_id
		)",
    );

    let entries = valid_commanders
        .iter()
        .map(|commander| (TrackedDeckCardType::Commander, 1, commander.card.oracle_id))
        .chain(valid_cards.iter().map(|entry| {
            (
                TrackedDeckCardType::Maindeck,
                entry.count.get() as i64,
                entry.card.card.oracle_id,
            )
        }));
    qb.push_values(entries, |mut row, entry| {
        row.push_bind(tracked_deck.uuid);
        row.push_bind(entry.0);
        row.push_bind(entry.1);
        row.push_bind(entry.2);
    });
    qb.build().execute(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}
