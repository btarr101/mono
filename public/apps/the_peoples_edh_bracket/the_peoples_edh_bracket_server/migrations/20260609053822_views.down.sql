DROP TABLE IF EXISTS card_ratings_cache;
DROP VIEW IF EXISTS card_rating_global;

DROP INDEX IF EXISTS idx_card_rating_rater_person_uuid_points;

DROP TRIGGER IF EXISTS trigger_rating_points_changed ON card_rating;
DROP TRIGGER IF EXISTS trigger_rating_insert_delete ON card_rating;

DROP FUNCTION IF EXISTS refresh_caches_on_rating_mutation();
DROP FUNCTION IF EXISTS refresh_card_ratings_cache_for_person(UUID, UUID, UUID);
DROP FUNCTION IF EXISTS refresh_person_ratings_cache_for_person(UUID);
DROP FUNCTION IF EXISTS calculate_global_points(NUMERIC, NUMERIC);

DROP TABLE IF EXISTS person_ratings_cache;
