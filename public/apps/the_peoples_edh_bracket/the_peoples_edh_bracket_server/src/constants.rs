pub const TS_RS_EXPORT_TO: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../the-peoples-edh-bracket-client/src/types/bindings/"
);

pub const MAX_TRACKED_DECKS_PER_PERSON: usize = 100;
pub const MAX_IN_FLIGHT_ANALYZE_REQUESTS: usize = 10;
