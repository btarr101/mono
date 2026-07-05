DROP TRIGGER IF EXISTS trigger_review_insert_delete ON card_rating_review;
DROP TRIGGER IF EXISTS trigger_review_liked_changed ON card_rating_review;
DROP FUNCTION IF EXISTS refresh_likes_dislikes_on_review_mutation();
DROP FUNCTION IF EXISTS refresh_person_likes_dislikes_cache(UUID);
DROP TABLE IF EXISTS person_likes_dislikes_cache;
