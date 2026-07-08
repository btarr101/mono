-- Remove old allocation/caching infrastructure first so the bulk UPDATE
-- below does not fire expensive cache-refresh triggers per row.
DROP TRIGGER IF EXISTS trigger_rating_points_changed ON card_rating;
DROP TRIGGER IF EXISTS trigger_rating_insert_delete ON card_rating;

-- Convert old allocation-style ppts to simple ratings using:
-- (ppts / max_ppts) * 10
-- where max_ppts is each rater's historical total allocated points.
WITH person_max_ppts AS (
    SELECT
        p.uuid AS person_uuid,
        COALESCE(SUM(cr.points), 0) AS max_ppts
    FROM person p
    LEFT JOIN card_rating cr ON cr.rater_person_uuid = p.uuid
    GROUP BY p.uuid
)
UPDATE card_rating cr
SET points = CASE
    WHEN pmp.max_ppts > 0 THEN (cr.points / pmp.max_ppts) * 10.0
    ELSE 0.0
END
FROM person_max_ppts pmp
WHERE cr.rater_person_uuid = pmp.person_uuid;

-- Enforce fixed 0-10 rating scale at the database layer.
ALTER TABLE card_rating
ADD CONSTRAINT card_rating_points_between_0_and_10
CHECK (points >= 0 AND points <= 10);

-- Remove remaining allocation/caching infrastructure.

DROP VIEW IF EXISTS card_rating_global;

DROP FUNCTION IF EXISTS refresh_caches_on_rating_mutation();
DROP FUNCTION IF EXISTS refresh_card_ratings_cache_for_person(UUID, UUID, UUID);
DROP FUNCTION IF EXISTS refresh_person_ratings_cache_for_person(UUID);
DROP FUNCTION IF EXISTS calculate_global_points(NUMERIC, NUMERIC);

DROP TABLE IF EXISTS global_ratings_state;
DROP TABLE IF EXISTS card_ratings_cache;
DROP TABLE IF EXISTS person_ratings_cache;

-- Keep these query-supporting indexes now that reads aggregate/filter directly from card_rating.
-- Re-create defensively in case they were removed previously.
CREATE INDEX IF NOT EXISTS idx_card_rating_rater_person_uuid_points
ON card_rating (rater_person_uuid, points);

CREATE INDEX IF NOT EXISTS idx_card_rating_card_oracle_id_points
ON card_rating (card_oracle_id, points);
