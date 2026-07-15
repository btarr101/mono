use std::{sync::Arc, time::Duration};

use reqwest::StatusCode;
use tokio::{
    sync::Mutex,
    time::{Instant, sleep},
};

use crate::moxfield::models::MoxfieldDeck;

pub struct MoxfieldClientConfig<'a> {
    pub user_agent: &'a str,
}

#[derive(Clone)]
pub struct MoxfieldClient {
    pub request_client: reqwest::Client,
    next_allowed_request: Arc<Mutex<Instant>>,
}

#[derive(thiserror::Error, Debug)]
pub enum MoxfieldDeckError {
    #[error("Deck not found")]
    NotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl MoxfieldClient {
    pub fn new(config: MoxfieldClientConfig<'_>) -> Self {
        let request_client = reqwest::Client::builder()
            .user_agent(config.user_agent)
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(reqwest::header::ACCEPT, "application/json".parse().expect("valid value"));
                headers
            })
            .build()
            .expect("valid client");

        Self {
            request_client,
            next_allowed_request: Mutex::new(Instant::now()).into(),
        }
    }

    /// Moxfield API has a strict rate limit of 1 request per second, and we want to respect
    /// that (otherwise we risk being blocked).
    ///
    /// We put it at 2 secs to be safe :)
    async fn wait_for_request_client(&self) -> &reqwest::Client {
        let mut next_allowed_request = self.next_allowed_request.lock().await;
        let now = Instant::now();

        let base = (*next_allowed_request).max(now);
        let wait_time = base - now;
        *next_allowed_request = base + Duration::from_secs(2);

        sleep(wait_time).await;

        &self.request_client
    }

    pub async fn deck(&self, deck_id: &str) -> Result<MoxfieldDeck, MoxfieldDeckError> {
        let url = format!("https://api2.moxfield.com/v3/decks/all/{}", deck_id);
        let response = self
            .wait_for_request_client()
            .await
            .get(&url)
            .send()
            .await
            .map(|response| response.error_for_status())
            .flatten()
            .map_err(|err| match err.status() {
                Some(StatusCode::NOT_FOUND) => MoxfieldDeckError::NotFound,
                _ => MoxfieldDeckError::Other(err.into()),
            })?;

        response
            .json::<MoxfieldDeck>()
            .await
            .map_err(|err| MoxfieldDeckError::Other(err.into()))
    }

    pub async fn export(&self, deck_id: &str, export_id: &str) -> anyhow::Result<String> {
        let url = format!("https://api2.moxfield.com/v2/decks/all/{}/export", deck_id);
        self.wait_for_request_client()
            .await
            .get(&url)
            .query(&[("arenaOnly", "false"), ("format", "plaintext"), ("exportId", export_id)])
            .send()
            .await?
            .text()
            .await
            .map_err(anyhow::Error::new)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::Config;

    #[tokio::test]
    async fn test_deck() -> anyhow::Result<()> {
        let config: Config = Config::from_env_with_dotenv()?;
        let client = MoxfieldClient::new(MoxfieldClientConfig {
            user_agent: config.moxfield_user_agent.as_str(),
        });

        let deck = client.deck("HCoQHbB3_0ePu9Va5e1pSQ").await?;

        dbg!(&deck);

        Ok(())
    }

    #[tokio::test]
    async fn test_rate_limited() -> anyhow::Result<()> {
        let config: Config = Config::from_env_with_dotenv()?;
        let client = MoxfieldClient::new(MoxfieldClientConfig {
            user_agent: config.moxfield_user_agent.as_str(),
        });

        let instant = Instant::now();
        for _ in 0..4 {
            client.deck("HCoQHbB3_0ePu9Va5e1pSQ").await?;
            dbg!(instant.elapsed().as_secs());
        }

        Ok(())
    }
}
