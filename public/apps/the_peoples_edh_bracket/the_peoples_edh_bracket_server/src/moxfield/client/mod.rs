use crate::moxfield::models::MoxfieldDeck;

pub struct MoxfieldClient {
    pub request_client: reqwest::Client,
    pub base_url: String,
}

impl MoxfieldClient {
    pub fn new() -> Self {
        let request_client = reqwest::Client::builder()
            .user_agent("the-peoples-edh-bracket/0.1")
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(reqwest::header::ACCEPT, "application/json".parse().expect("valid value"));
                headers
            })
            .build()
            .expect("valid client");

        Self {
            request_client,
            base_url: "https://api2.moxfield.com/v3".to_string(),
        }
    }

    pub async fn deck(&self, deck_id: &str) -> anyhow::Result<MoxfieldDeck> {
        let url = format!("https://api2.moxfield.com/v3/decks/all/{}", deck_id);
        let response = self.request_client.get(&url).send().await?;

        response.json::<MoxfieldDeck>().await.map_err(anyhow::Error::new)
    }

    pub async fn export(&self, deck_id: &str, export_id: &str) -> anyhow::Result<String> {
        let url = format!("https://api2.moxfield.com/v2/decks/all/{}/export", deck_id);
        self.request_client
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

    #[tokio::test]
    async fn test_deck() -> anyhow::Result<()> {
        let client = MoxfieldClient::new();
        let deck = client.deck("HCoQHbB3_0ePu9Va5e1pSQ").await?;

        dbg!(&deck);

        Ok(())
    }

    #[tokio::test]
    async fn test_export() -> anyhow::Result<()> {
        let client = MoxfieldClient::new();
        let deck = client.deck("HCoQHbB3_0ePu9Va5e1pSQ").await?;
        let export = client.export("HCoQHbB3_0ePu9Va5e1pSQ", &deck.export_id).await?;

        dbg!(&export);

        Ok(())
    }
}
