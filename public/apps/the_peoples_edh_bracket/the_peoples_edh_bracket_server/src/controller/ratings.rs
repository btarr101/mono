use bigdecimal::BigDecimal;
use sqlx::PgPool;

use crate::model::card_rating::CardRating;

pub struct RateCardParams {
    pub card_oracle_id: uuid::Uuid,
    pub person_uuid: uuid::Uuid,
    pub points: BigDecimal,
    pub reason: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum RateCardError {
    #[error("Card does not exist")]
    CardDoesNotExist,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<sqlx::Error> for RateCardError {
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref db_err) = e
            && db_err.constraint() == Some("card_rating_card_oracle_id_fkey")
        {
            return Self::CardDoesNotExist;
        }
        Self::Other(anyhow::Error::new(e))
    }
}

pub async fn rate_card(
    RateCardParams {
        card_oracle_id,
        person_uuid,
        points,
        reason,
    }: RateCardParams,
    pg_pool: &PgPool,
) -> Result<CardRating, RateCardError> {
    let card_rating = sqlx::query_as!(
        CardRating,
        "INSERT INTO card_rating (card_oracle_id, rater_person_uuid, points, reason)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (card_oracle_id, rater_person_uuid)
        DO UPDATE SET
            points = EXCLUDED.points,
            reason = EXCLUDED.reason
        RETURNING *
        ",
        card_oracle_id,
        person_uuid,
        points.to_owned(),
        reason,
    )
    .fetch_one(pg_pool)
    .await?;

    Ok(card_rating)
}

pub struct ReviewRatingParams {
    pub card_rating_uuid: uuid::Uuid,
    pub person_uuid: uuid::Uuid,
    pub like: Option<bool>,
}

#[derive(thiserror::Error, Debug)]
pub enum ReviewRatingError {
    #[error("Rating does not exist")]
    RatingDoesNotExist,
    #[error("Person cannot review their own rating")]
    CannotReviewOwnRating,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<sqlx::Error> for ReviewRatingError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(db) if db.constraint() == Some("card_rating_review_reviewed_card_rating_uuid_fkey") => {
                ReviewRatingError::RatingDoesNotExist
            }
            sqlx::Error::Database(db) if db.message() == "self_review" => ReviewRatingError::CannotReviewOwnRating,
            _ => ReviewRatingError::Other(error.into()),
        }
    }
}

pub async fn review_rating(params: ReviewRatingParams, pg_pool: &PgPool) -> Result<(), ReviewRatingError> {
    if let Some(like) = params.like {
        sqlx::query!(
            "INSERT INTO card_rating_review (reviewer_person_uuid, reviewed_card_rating_uuid, liked)
             VALUES ($1, $2, $3)
             ON CONFLICT (reviewer_person_uuid, reviewed_card_rating_uuid)
             DO UPDATE SET liked = EXCLUDED.liked",
            params.person_uuid,
            params.card_rating_uuid,
            like,
        )
        .execute(pg_pool)
        .await?;
    } else {
        sqlx::query!(
            "DELETE FROM card_rating_review
            WHERE reviewer_person_uuid = $1 AND reviewed_card_rating_uuid = $2",
            params.person_uuid,
            params.card_rating_uuid
        )
        .execute(pg_pool)
        .await?;
    }

    Ok(())
}
