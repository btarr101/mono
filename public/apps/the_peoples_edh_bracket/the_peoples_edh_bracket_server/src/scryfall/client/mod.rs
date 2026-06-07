use crate::scryfall::client::{bulk_data::ScryfallBulkDataRequestBuilder, cards::ScryfallCardsRequestBuilder};

pub mod bulk_data;
pub mod cards;

#[derive(Clone)]
pub struct ScryfallClient {
    pub base_url: &'static str,
    pub request_client: reqwest::Client,
}

impl Default for ScryfallClient {
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

impl ScryfallClient {
    pub fn new() -> Self { Self::default() }

    pub fn cards(&self) -> ScryfallCardsRequestBuilder<'_> { ScryfallCardsRequestBuilder::new(self) }
    pub fn bulk_data(&self) -> ScryfallBulkDataRequestBuilder<'_> { ScryfallBulkDataRequestBuilder::new(self) }
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
        let list = client
            .cards()
            .search(ScryfallCardsSearchParams { q: "emrakul" })
            .await
            .unwrap();

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
                        oracle_id: &uuid!("96e5d4a1-e59f-4140-823f-a17e15ee5d8d"),
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

    #[tokio::test]
    async fn test_bulk_data() {
        let client = ScryfallClient::new();
        client.bulk_data().ty("oracle-cards").await.unwrap();
    }
}
