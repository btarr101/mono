DROP TRIGGER IF EXISTS update_person_updated ON person;
DROP TRIGGER IF EXISTS update_card_rating_updated ON card_rating;

ALTER TABLE person
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

ALTER TABLE card_rating
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

DROP FUNCTION IF EXISTS update_updated_column();
