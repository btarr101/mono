use std::path::Path;

use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde_inline_default("127.0.0.1:3001".to_string())]
    pub bind_address: String,
    #[serde_inline_default(if cfg!(debug_assertions) { "dev".to_string() } else { "prod".to_string() })]
    pub stage: String,
    // I am lazy... forgive me.
    #[serde_inline_default(if cfg!(debug_assertions) { "971820376877-6ficbee232172n33gnjgi4f5gab5n5fn.apps.googleusercontent.com".to_string() } else { "971820376877-fmlngfalfttccr6c4597dqbbrj9vjs9n.apps.googleusercontent.com".to_string() })]
    pub google_oauth_client_id: String,
    pub moxfield_user_agent: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env_with_dotenv() -> Result<Self, serde_envfile::Error> {
        #[cfg(debug_assertions)]
        {
            let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
            let _ = dotenvy::from_filename(&env_path);
        }

        let _ = dotenvy::dotenv();
        serde_envfile::from_env()
    }
}
