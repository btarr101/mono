use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde_inline_default("127.0.0.1:3000".to_string())]
    pub bind_address: String,
    #[serde_inline_default(if cfg!(debug_assertions) { "dev".to_string() } else { "prod".to_string() })]
    pub stage: String,
    pub google_oauth_client_id: String,
}
