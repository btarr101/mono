CREATE TABLE IF NOT EXISTS person_likes_dislikes_cache (
    person_uuid UUID PRIMARY KEY REFERENCES person(uuid) ON DELETE CASCADE,
    likes       BIGINT NOT NULL DEFAULT 0,
    dislikes    BIGINT NOT NULL DEFAULT 0
);

CREATE OR REPLACE FUNCTION refresh_person_likes_dislikes_cache(p_person_uuid UUID)
RETURNS VOID AS $$
BEGIN
    IF p_person_uuid IS NULL THEN
        RAISE EXCEPTION
            'refresh_person_likes_dislikes_cache called with NULL person_uuid';
    END IF;

    INSERT INTO person_likes_dislikes_cache (person_uuid, likes, dislikes)
    SELECT
        p_person_uuid,
        COALESCE(SUM(CASE WHEN liked THEN 1 ELSE 0 END), 0) AS likes,
        COALESCE(SUM(CASE WHEN NOT liked THEN 1 ELSE 0 END), 0) AS dislikes
    FROM card_rating_review
    INNER JOIN card_rating cr
        ON card_rating_review.reviewed_card_rating_uuid = cr.uuid
    WHERE cr.rater_person_uuid = p_person_uuid
    ON CONFLICT (person_uuid)
    DO UPDATE SET
        likes = EXCLUDED.likes,
        dislikes = EXCLUDED.dislikes;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION refresh_likes_dislikes_on_review_mutation()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM refresh_person_likes_dislikes_cache(
        (
            SELECT rater_person_uuid
            FROM card_rating
            WHERE uuid = COALESCE(
                NEW.reviewed_card_rating_uuid,
                OLD.reviewed_card_rating_uuid
            )
        )
    );

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_review_insert_delete ON card_rating_review;
DROP TRIGGER IF EXISTS trigger_review_liked_changed ON card_rating_review;

CREATE TRIGGER trigger_review_insert_delete
AFTER INSERT OR DELETE ON card_rating_review
FOR EACH ROW
EXECUTE FUNCTION refresh_likes_dislikes_on_review_mutation();

CREATE TRIGGER trigger_review_liked_changed
AFTER UPDATE OF liked ON card_rating_review
FOR EACH ROW
WHEN (OLD.liked IS DISTINCT FROM NEW.liked)
EXECUTE FUNCTION refresh_likes_dislikes_on_review_mutation();
