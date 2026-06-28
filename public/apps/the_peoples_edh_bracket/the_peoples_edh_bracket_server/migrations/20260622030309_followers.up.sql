CREATE TABLE IF NOT EXISTS follower (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_person_uuid UUID NOT NULL references person(uuid) ON DELETE CASCADE,
    followed_person_uuid UUID NOT NULL references person(uuid) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    UNIQUE (follower_person_uuid, followed_person_uuid)
)
