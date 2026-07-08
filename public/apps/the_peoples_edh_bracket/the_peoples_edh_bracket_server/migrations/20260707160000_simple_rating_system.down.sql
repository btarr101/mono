-- NOTE: lossy rollback.
-- The up migration converted historical allocation-style values into a fixed 0–10 per-card scale
-- using per-user totals. The original allocation totals cannot be reconstructed from current data.
-- We intentionally do not transform card_rating.points here.

-- Remove the fixed 0-10 guard introduced by the up migration.
ALTER TABLE card_rating
DROP CONSTRAINT IF EXISTS card_rating_points_between_0_and_10;

DROP VIEW IF EXISTS global_ratings_state;
DROP VIEW IF EXISTS card_ratings_cache;
DROP VIEW IF EXISTS card_rating_global;
DROP VIEW IF EXISTS person_ratings_cache;

DROP FUNCTION IF EXISTS calculate_global_points(NUMERIC, NUMERIC);

-- Restore old allocation/cache infrastructure.
CREATE TABLE IF NOT EXISTS global_ratings_state (
    id BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (id),
    unrated_card_rank INT NOT NULL DEFAULT 1
);
INSERT INTO global_ratings_state DEFAULT VALUES ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS person_ratings_cache (
    person_uuid UUID PRIMARY KEY REFERENCES person(uuid) ON DELETE CASCADE,
    total_personal_points NUMERIC NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS card_ratings_cache (
    card_oracle_id UUID PRIMARY KEY REFERENCES card(oracle_id) ON DELETE CASCADE,
    average_global_points NUMERIC NOT NULL DEFAULT 0.0,
    card_rank INT NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_card_rating_rater_person_uuid_points
ON card_rating (rater_person_uuid, points);

CREATE INDEX IF NOT EXISTS idx_card_rating_card_oracle_id_points
ON card_rating (card_oracle_id, points);

CREATE OR REPLACE FUNCTION calculate_global_points(
    p_points NUMERIC,
    p_total_personal_points NUMERIC
)
RETURNS NUMERIC AS $$
BEGIN
    IF p_total_personal_points <= 0 THEN
        RETURN 0.0;
    END IF;

    RETURN (p_points / p_total_personal_points) * 10.0;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

CREATE OR REPLACE FUNCTION refresh_person_ratings_cache_for_person(p_person_uuid UUID)
RETURNS VOID AS $$
BEGIN
    INSERT INTO person_ratings_cache (person_uuid, total_personal_points)
    SELECT
        p_person_uuid,
        COALESCE(SUM(points), 1)
    FROM card_rating
    WHERE rater_person_uuid = p_person_uuid
    ON CONFLICT (person_uuid)
    DO UPDATE SET
        total_personal_points = EXCLUDED.total_personal_points;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION refresh_card_ratings_cache_for_person(
    p_person_uuid UUID,
    p_new_card_oracle_id UUID,
    p_old_card_oracle_id UUID
)
RETURNS VOID AS $$
BEGIN
    WITH affected_cards AS (
        SELECT card_oracle_id
        FROM card_rating
        WHERE rater_person_uuid = p_person_uuid
        UNION
        SELECT p_new_card_oracle_id
        WHERE p_new_card_oracle_id IS NOT NULL
        UNION
        SELECT p_old_card_oracle_id
        WHERE p_old_card_oracle_id IS NOT NULL
    ),
    aggregated AS (
        SELECT
            ac.card_oracle_id,
            AVG(calculate_global_points(cr.points, prc.total_personal_points))
                AS average_global_points
        FROM affected_cards ac
        LEFT JOIN card_rating cr ON cr.card_oracle_id = ac.card_oracle_id
        LEFT JOIN person_ratings_cache prc ON prc.person_uuid = cr.rater_person_uuid
        GROUP BY ac.card_oracle_id
    )
    INSERT INTO card_ratings_cache (card_oracle_id, average_global_points)
    SELECT
        a.card_oracle_id,
        COALESCE(a.average_global_points, 0.0)
    FROM aggregated a
    ON CONFLICT (card_oracle_id)
    DO UPDATE SET
        average_global_points = EXCLUDED.average_global_points;

    UPDATE card_ratings_cache crc
    SET card_rank = ranked.new_rank
    FROM (
        SELECT
            card_oracle_id,
            DENSE_RANK() OVER (ORDER BY average_global_points DESC) AS new_rank
        FROM card_ratings_cache
    ) ranked
    WHERE crc.card_oracle_id = ranked.card_oracle_id;

    UPDATE global_ratings_state
    SET unrated_card_rank = 1 + (
        SELECT COUNT(DISTINCT average_global_points)::int
        FROM card_ratings_cache
        WHERE average_global_points > 0.0
    );
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION refresh_caches_on_rating_mutation()
RETURNS TRIGGER AS $$
DECLARE
    v_person_uuid UUID;
    v_new_card_oracle_id UUID;
    v_old_card_oracle_id UUID;
BEGIN
    v_person_uuid := COALESCE(NEW.rater_person_uuid, OLD.rater_person_uuid);
    v_new_card_oracle_id := CASE WHEN TG_OP IN ('INSERT', 'UPDATE') THEN NEW.card_oracle_id ELSE NULL END;
    v_old_card_oracle_id := CASE WHEN TG_OP IN ('DELETE', 'UPDATE') THEN OLD.card_oracle_id ELSE NULL END;

    PERFORM refresh_person_ratings_cache_for_person(v_person_uuid);
    PERFORM refresh_card_ratings_cache_for_person(v_person_uuid, v_new_card_oracle_id, v_old_card_oracle_id);

    IF (TG_OP = 'DELETE') THEN
        RETURN OLD;
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_rating_insert_delete
AFTER INSERT OR DELETE ON card_rating
FOR EACH ROW
EXECUTE FUNCTION refresh_caches_on_rating_mutation();

CREATE TRIGGER trigger_rating_points_changed
AFTER UPDATE OF points ON card_rating
FOR EACH ROW
WHEN (OLD.points IS DISTINCT FROM NEW.points)
EXECUTE FUNCTION refresh_caches_on_rating_mutation();

CREATE OR REPLACE VIEW card_rating_global AS
SELECT
    cr.uuid AS card_rating_uuid,
    cr.card_oracle_id,
    cr.rater_person_uuid AS person_uuid,
    calculate_global_points(cr.points, prc.total_personal_points) AS global_points
FROM card_rating cr
JOIN person_ratings_cache prc ON prc.person_uuid = cr.rater_person_uuid;

-- Backfill restored cache tables.
INSERT INTO person_ratings_cache (person_uuid, total_personal_points)
SELECT
    p.uuid,
    COALESCE(SUM(cr.points), 1)
FROM person p
LEFT JOIN card_rating cr ON cr.rater_person_uuid = p.uuid
GROUP BY p.uuid
ON CONFLICT (person_uuid)
DO UPDATE SET total_personal_points = EXCLUDED.total_personal_points;

INSERT INTO card_ratings_cache (card_oracle_id, average_global_points)
SELECT
    c.oracle_id,
    COALESCE(AVG(crg.global_points), 0.0)
FROM card c
LEFT JOIN card_rating_global crg ON crg.card_oracle_id = c.oracle_id
GROUP BY c.oracle_id
ON CONFLICT (card_oracle_id)
DO UPDATE SET average_global_points = EXCLUDED.average_global_points;

UPDATE card_ratings_cache crc
SET card_rank = ranked.new_rank
FROM (
    SELECT
        card_oracle_id,
        DENSE_RANK() OVER (ORDER BY average_global_points DESC) AS new_rank
    FROM card_ratings_cache
) ranked
WHERE crc.card_oracle_id = ranked.card_oracle_id;

UPDATE global_ratings_state
SET unrated_card_rank = 1 + (
    SELECT COUNT(DISTINCT average_global_points)::int
    FROM card_ratings_cache
    WHERE average_global_points > 0.0
);
