use bigdecimal::{BigDecimal, FromPrimitive};
use cgisf_lib::{SentenceConfigBuilder, gen_sentence};
use futures_util::future::try_join_all;
use itertools::Itertools;
use tracing::info;

use crate::{
    controller::{
        persons::create_debug_person,
        ratings::{RateCardParams, rate_card},
    },
    state::AppState,
};

pub async fn seed(state: AppState) -> anyhow::Result<()> {
    info!("Starting random seed...");

    info!("Creating random people...");
    let persons = try_join_all((0..100).map(|_| create_debug_person(&state.pg_pool))).await?;

    info!("Getting random cards...");
    let cards = sqlx::query!(
        "SELECT name, oracle_id
		FROM card
		ORDER BY RANDOM()
		LIMIT 10"
    )
    .fetch_all(&state.pg_pool)
    .await?;

    info!("Rating cards...");
    let config = SentenceConfigBuilder::random().build();
    for (person, card) in persons.iter().cartesian_product(cards.iter()) {
        info!("'{}' rating '{}'...", person.username, card.name);

        rate_card(
            RateCardParams {
                card_oracle_id: card.oracle_id,
                person_uuid: person.uuid,
                points: BigDecimal::from_f64(fastrand::f64() * 10.).unwrap_or_default(),
                reason: Some(gen_sentence(config)),
            },
            &state.pg_pool,
        )
        .await?;
    }

    info!("Done!");

    Ok(())
}
