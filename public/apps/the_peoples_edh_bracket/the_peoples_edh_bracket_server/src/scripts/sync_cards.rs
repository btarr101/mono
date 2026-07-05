use sqlx::QueryBuilder;
use tracing::info;

use crate::{model::card::CardLegality, scryfall::models::ScryfallLegality, state::AppState};

pub async fn sync_cards(state: AppState) -> anyhow::Result<()> {
    sync_oracle_cards(&state).await?;
    sync_alternate_card_names(&state).await?;

    info!("Finished!");

    Ok(())
}

pub async fn sync_oracle_cards(state: &AppState) -> anyhow::Result<()> {
    info!("Starting cards sync...");
    let bulk_data_meta_request_builder = state.scryfall_client.bulk_data().ty("oracle-cards").await?;

    info!(
        "Starting bulk data fetch from '{}'",
        bulk_data_meta_request_builder.meta().download_uri,
    );

    let mut cards = bulk_data_meta_request_builder.data().await?;
    cards.retain(|card| card.oracle_id.is_some() && !matches!(card.legalities.commander, ScryfallLegality::NotLegal));

    for (idx, cards_chunk) in cards.chunks(1000).enumerate() {
        info!("Handling chunk '{}'", idx);

        let mut qb = QueryBuilder::new(
            "INSERT INTO card (
				oracle_id,
				name,
				image_uri,
				legality
			)",
        );

        qb.push_values(cards_chunk, |mut row, card| {
            row.push_bind(card.oracle_id)
                .push_bind(&card.name)
                .push_bind(card.mediumest_image_uri())
                .push_bind(match card.legalities.commander {
                    ScryfallLegality::Legal => CardLegality::Legal,
                    ScryfallLegality::Restricted => CardLegality::Restricted,
                    _ => CardLegality::Banned,
                });
        });

        qb.push(
            " ON CONFLICT (oracle_id)
            DO UPDATE SET
                name = EXCLUDED.name,
                image_uri = EXCLUDED.image_uri,
                legality = EXCLUDED.legality",
        );

        qb.build().execute(&state.pg_pool).await?;
    }

    Ok(())
}

pub async fn sync_alternate_card_names(state: &AppState) -> anyhow::Result<()> {
    info!("Starting alternate card names sync...");

    let bulk_data_meta_request_builder = state.scryfall_client.bulk_data().ty("unique-artwork").await?;
    info!(
        "Starting unique-artwork bulk data fetch from '{}'",
        bulk_data_meta_request_builder.meta().download_uri,
    );

    let cards = bulk_data_meta_request_builder.data().await?;
    let cards_len = cards.len();

    let alternate_names = cards
        .into_iter()
        .filter_map(|card| {
            card.oracle_id
                .map(|oracle_id| (oracle_id, card.name, card.flavor_name, card.printed_name))
        })
        .flat_map(|(oracle_id, name, flavor_name, printed_name)| {
            [Some(name), flavor_name, printed_name]
                .into_iter()
                .flatten()
                .map(move |name| (oracle_id, name))
        })
        .collect::<Vec<_>>();

    info!("Fetched '{}' unique-artwork cards", cards_len);
    info!("Prepared '{}' alternate card names", alternate_names.len());

    for (idx, alternate_names_chunk) in alternate_names.chunks(1000).enumerate() {
        info!("Handling alternate name chunk '{}'", idx);

        let mut qb = QueryBuilder::new("INSERT INTO alternate_card_name (card_oracle_id, name) ");

        qb.push("SELECT source.card_oracle_id, source.name FROM (");
        qb.push_values(alternate_names_chunk, |mut row, (oracle_id, name)| {
            row.push_bind(oracle_id).push_bind(name);
        });
        qb.push(
            ") AS source(card_oracle_id, name)
            INNER JOIN card ON card.oracle_id = source.card_oracle_id
            WHERE card.name <> source.name
            ON CONFLICT (card_oracle_id, name) DO NOTHING",
        );

        qb.build().execute(&state.pg_pool).await?;
    }

    Ok(())
}
