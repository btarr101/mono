use crate::scryfall::{
    client::{ScryfallClient, bulk_data::fetched::ScryfallFetchedBulkDataMetaRequestBuilder},
    models::ScryfallBulkDataMeta,
};

pub mod fetched;

pub struct ScryfallBulkDataRequestBuilder<'a> {
    base_url: String,
    client: &'a ScryfallClient,
}

impl<'a> ScryfallBulkDataRequestBuilder<'a> {
    pub(super) fn new(client: &'a ScryfallClient) -> Self {
        Self {
            client,
            base_url: format!("{}/bulk-data", client.base_url),
        }
    }

    pub async fn ty(self, ty: &'_ str) -> anyhow::Result<ScryfallFetchedBulkDataMetaRequestBuilder<'a>> {
        let url = format!("{}/{}", self.base_url, ty);
        let bulk_data = self
            .client
            .request_client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<ScryfallBulkDataMeta>()
            .await?;

        Ok(ScryfallFetchedBulkDataMetaRequestBuilder::new(self.client, bulk_data))
    }
}
