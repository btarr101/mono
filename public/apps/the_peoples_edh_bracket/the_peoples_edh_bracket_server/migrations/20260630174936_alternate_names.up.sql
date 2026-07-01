CREATE TABLE alternate_card_name (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    card_oracle_id UUID NOT NULL REFERENCES card(oracle_id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    UNIQUE (card_oracle_id, name)
);
