use crate::scryfall::client::cards::ScryfallCardsRequestBuilder;

pub mod cards;

#[derive(Clone)]
pub struct ScryfallClient<'a> {
    pub base_url: &'a str,
    pub request_client: reqwest::Client,
}

impl<'a> Default for ScryfallClient<'a> {
    fn default() -> Self {
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
            base_url: "https://api.scryfall.com",
            request_client,
        }
    }
}

impl<'a> ScryfallClient<'a> {
    pub fn new() -> Self { Self::default() }

    pub fn cards<'b>(&'b self) -> ScryfallCardsRequestBuilder<'a, 'b> { ScryfallCardsRequestBuilder::new(self) }
}

#[cfg(test)]
mod test {
    use uuid::uuid;

    use super::*;
    use crate::scryfall::client::cards::{
        ScryfallCardsCollectionEntry, ScryfallCardsCollectionParams, ScryfallCardsSearchParams,
    };

    #[tokio::test]
    async fn test_cards_search() {
        let client = ScryfallClient::new();
        let list = client.cards().search(ScryfallCardsSearchParams { q: "jace" }).await.unwrap();

        dbg!("{:?}", &list);

        assert!(!list.data.is_empty());
    }

    #[tokio::test]
    async fn test_cards_collection() {
        let client = ScryfallClient::new();
        let list = client
            .cards()
            .collection(ScryfallCardsCollectionParams {
                identifiers: &[
                    ScryfallCardsCollectionEntry {
                        oracle_id: &uuid!("9aa0d3cc-0785-4b37-a495-33f4bf4114ef"),
                    },
                    ScryfallCardsCollectionEntry {
                        oracle_id: &uuid!("ec17b9db-dd80-4b46-a1b5-215f26bf6498"),
                    },
                ],
            })
            .await
            .unwrap();

        dbg!("{:?}", &list);

        assert!(!list.data.is_empty());
    }
}
