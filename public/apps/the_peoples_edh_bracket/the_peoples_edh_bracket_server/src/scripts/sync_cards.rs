use sqlx::QueryBuilder;
use tracing::info;

use crate::{model::card::CardLegality, scryfall::models::ScryfallLegality, state::AppState};

pub async fn sync_cards(state: AppState) -> anyhow::Result<()> {
    info!("Starting cards sync...");
    let bulk_data_meta_request_builder = state.scryfall_client.bulk_data().ty("oracle-cards").await?;

    info!(
        "Starting bulk data fetch from '{}'",
        bulk_data_meta_request_builder.meta().download_uri,
    );

    let mut cards = bulk_data_meta_request_builder.data().await?;
    cards.retain(|card| !matches!(card.legalities.commander, ScryfallLegality::NotLegal));

    for (idx, cards_chunk) in cards.chunks(1000).enumerate() {
        info!("Handling chunk '{}'", idx);

        let mut qb = QueryBuilder::new(
            r#"
			INSERT INTO card (
				oracle_id,
				name,
				image_uri,
				legality
			)
			"#,
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

    info!("Finished!");

    Ok(())
}
