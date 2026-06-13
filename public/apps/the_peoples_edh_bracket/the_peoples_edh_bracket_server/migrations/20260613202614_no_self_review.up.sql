CREATE OR REPLACE FUNCTION check_no_self_review()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM card_rating
        WHERE uuid = NEW.reviewed_card_rating_uuid
          AND rater_person_uuid = NEW.reviewer_person_uuid
    ) THEN
        RAISE EXCEPTION 'self_review' USING ERRCODE = 'P0001';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER no_self_review
BEFORE INSERT OR UPDATE ON card_rating_review
FOR EACH ROW EXECUTE FUNCTION check_no_self_review();
