CREATE OR REPLACE FUNCTION update_updated_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


ALTER TABLE person 
ADD COLUMN created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
ADD COLUMN updated_at TIMESTAMPTZ;

CREATE TRIGGER update_person_updated
    BEFORE UPDATE ON person
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_column();


ALTER TABLE card_rating 
ADD COLUMN created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
ADD COLUMN updated_at TIMESTAMPTZ;

CREATE TRIGGER update_card_rating_updated
    BEFORE UPDATE ON card_rating
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_column();

