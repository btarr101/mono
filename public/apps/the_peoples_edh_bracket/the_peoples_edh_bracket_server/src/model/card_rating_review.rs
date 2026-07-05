#[derive(sqlx::FromRow)]
pub struct CardRatingReview {
    pub uuid: uuid::Uuid,
    pub card_oracle_id: uuid::Uuid,
    pub rater_person_uuid: uuid::Uuid,
    pub liked: bool,
}
