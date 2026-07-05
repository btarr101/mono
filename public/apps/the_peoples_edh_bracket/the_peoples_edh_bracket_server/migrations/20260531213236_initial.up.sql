CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE IF NOT EXISTS person (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid()
);

CREATE TABLE IF NOT EXISTS card (
  oracle_id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  image_uri TEXT,
  legality TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS card_rating (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  card_oracle_id UUID NOT NULL REFERENCES card(oracle_id) ON DELETE CASCADE,
  rater_person_uuid UUID NOT NULL REFERENCES person(uuid) ON DELETE CASCADE,
  points NUMERIC NOT NULL CHECK (points >= 0),
  reason TEXT,
  UNIQUE (card_oracle_id, rater_person_uuid)
);

CREATE TABLE IF NOT EXISTS card_rating_review (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  reviewer_person_uuid UUID NOT NULL REFERENCES person(uuid) ON DELETE CASCADE,
  reviewed_card_rating_uuid UUID NOT NULL REFERENCES card_rating(uuid) ON DELETE CASCADE,
  liked BOOL NOT NULL,
  UNIQUE (reviewer_person_uuid, reviewed_card_rating_uuid)
);
