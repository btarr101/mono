CREATE TABLE IF NOT EXISTS tracked_deck (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tracker_person_uuid UUID NOT NULL references person(uuid) ON DELETE CASCADE,
    name TEXT NOT NULL,
    url_source TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ
);

CREATE TRIGGER update_tracked_deck_updated
    BEFORE UPDATE ON tracked_deck
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_column();

CREATE TABLE IF NOT EXISTS tracked_deck_card (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tracked_deck_uuid UUID NOT NULL references tracked_deck(uuid) ON DELETE CASCADE,
    ty TEXT NOT NULL,
    count INTEGER NOT NULL CHECK (count >= 1),
    card_oracle_id UUID NOT NULL references card(oracle_id) ON DELETE CASCADE,
    UNIQUE (tracked_deck_uuid, card_oracle_id)
);
