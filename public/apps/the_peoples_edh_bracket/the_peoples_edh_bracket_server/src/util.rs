use std::num::NonZeroUsize;

use sqlx::PgPool;

use crate::model::card::{Card, CardLegality, CardWithGlobalPoints};

pub fn parse_pagination(page: NonZeroUsize, page_size: NonZeroUsize) -> (i64, i64) {
    let limit = page_size.get() as i64;
    let offset = ((page.get() - 1) * page_size.get()) as i64;

    (limit, offset)
}

pub async fn find_cards_by_names_with_alternate_name(
    cards_names: impl IntoIterator<Item = impl AsRef<str>>,
    pg_pool: &PgPool,
) -> anyhow::Result<(Vec<(CardWithGlobalPoints, Option<String>)>, Vec<String>)> {
    let input = cards_names.into_iter().map(|n| n.as_ref().to_string()).collect::<Vec<_>>();
    let lowercased = input.iter().map(|n| n.to_lowercase()).collect::<Vec<_>>();

    let cards = sqlx::query!(
        "WITH matched_cards AS (
            SELECT DISTINCT c.oracle_id
            FROM card c
            LEFT JOIN LATERAL (
                SELECT name AS alternate_name
                FROM alternate_card_name acn
                WHERE acn.card_oracle_id = c.oracle_id
                    AND LOWER(acn.name) = ANY($1)
                LIMIT 1
            ) acn ON TRUE
            WHERE LOWER(c.name) = ANY($1)
               OR acn.alternate_name IS NOT NULL
        ),
        card_global_points AS (
            SELECT
                cr.card_oracle_id,
                AVG(cr.points) AS average_global_points
            FROM card_rating cr
            INNER JOIN matched_cards mc ON mc.oracle_id = cr.card_oracle_id
            GROUP BY cr.card_oracle_id
        )
        SELECT
            c.oracle_id as \"oracle_id!\",
            c.name as \"name!\",
            c.image_uri,
            c.legality as \"legality!: CardLegality\",
            COALESCE(cgp.average_global_points, 0.0) as \"global_points!\",
            acn.alternate_name
        FROM matched_cards mc
        INNER JOIN card c ON c.oracle_id = mc.oracle_id
        LEFT JOIN card_global_points cgp ON c.oracle_id = cgp.card_oracle_id
        LEFT JOIN LATERAL (
            SELECT name AS alternate_name
            FROM alternate_card_name acn
            WHERE acn.card_oracle_id = c.oracle_id
                AND LOWER(acn.name) = ANY($1)
            LIMIT 1
        ) acn ON TRUE
        ",
        &lowercased
    )
    .fetch_all(pg_pool)
    .await?
    .into_iter()
    .map(|row| {
        (
            row.alternate_name,
            CardWithGlobalPoints {
                card: Card {
                    oracle_id: row.oracle_id,
                    name: row.name,
                    image_uri: row.image_uri,
                    legality: row.legality,
                },
                global_points: row.global_points,
            },
        )
    })
    .collect::<Vec<_>>();

    let invalid_card_names = input
        .iter()
        .filter(|name| {
            !cards
                .iter()
                .find(|(alternate_name, card)| {
                    card.card.name.eq_ignore_ascii_case(name.as_str())
                        || alternate_name
                            .as_deref()
                            .map_or(false, |alternate_name| alternate_name.eq_ignore_ascii_case(name.as_str()))
                })
                .is_some()
        })
        .cloned()
        .collect::<Vec<_>>();

    let cards = cards
        .into_iter()
        .map(|(alternate_name, card)| (card, alternate_name))
        .collect::<Vec<_>>();

    Ok((cards, invalid_card_names))
}

pub async fn find_cards_by_names(
    cards_names: impl IntoIterator<Item = impl AsRef<str>>,
    pg_pool: &PgPool,
) -> anyhow::Result<(Vec<CardWithGlobalPoints>, Vec<String>)> {
    let (cards_with_alternate_name, invalid_card_names) = find_cards_by_names_with_alternate_name(cards_names, pg_pool).await?;

    let cards = cards_with_alternate_name
        .into_iter()
        .map(|(card, _)| card)
        .collect::<Vec<_>>();

    Ok((cards, invalid_card_names))
}
