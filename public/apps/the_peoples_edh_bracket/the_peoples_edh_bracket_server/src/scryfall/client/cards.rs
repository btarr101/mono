use serde::Serialize;

use crate::scryfall::{
    client::ScryfallClient,
    models::{ScryfallCard, ScryfallList},
};

pub struct ScryfallCardsRequestBuilder<'a> {
    base_url: String,
    client: &'a ScryfallClient,
}

#[derive(Serialize)]
pub struct ScryfallCardsSearchParams<'a> {
    pub q: &'a str,
}

#[derive(Serialize)]
pub struct ScryfallCardsCollectionParams<'a> {
    pub identifiers: &'a [ScryfallCardsCollectionEntry<'a>],
}

#[derive(Serialize)]
pub struct ScryfallCardsCollectionEntry<'a> {
    pub oracle_id: &'a uuid::Uuid,
}

impl<'a> ScryfallCardsRequestBuilder<'a> {
    pub(super) fn new(client: &'a ScryfallClient) -> Self {
        Self {
            client,
            base_url: format!("{}/cards", client.base_url),
        }
    }

    pub async fn search(self, params: ScryfallCardsSearchParams<'_>) -> anyhow::Result<ScryfallList<ScryfallCard>> {
        let url = format!("{}/search", self.base_url);

        let response = self.client.request_client.get(url).query(&params).send().await?;

        response
            .error_for_status()?
            .json::<ScryfallList<ScryfallCard>>()
            .await
            .map_err(anyhow::Error::new)
    }

    pub async fn collection(self, params: ScryfallCardsCollectionParams<'_>) -> anyhow::Result<ScryfallList<ScryfallCard>> {
        let url = format!("{}/collection", self.base_url);

        self.client
            .request_client
            .post(url)
            .json(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<ScryfallList<ScryfallCard>>()
            .await
            .map_err(anyhow::Error::new)
    }
}
