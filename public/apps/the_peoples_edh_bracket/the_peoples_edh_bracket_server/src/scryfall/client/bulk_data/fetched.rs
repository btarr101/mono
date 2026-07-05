use std::path::Path;

use futures_util::TryStreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::scryfall::{
    client::ScryfallClient,
    models::{ScryfallBulkDataMeta, ScryfallCard},
};

pub struct ScryfallFetchedBulkDataMetaRequestBuilder<'a> {
    client: &'a ScryfallClient,
    bulk_data: ScryfallBulkDataMeta,
}

impl<'a> ScryfallFetchedBulkDataMetaRequestBuilder<'a> {
    pub(super) fn new(client: &'a ScryfallClient, bulk_data: ScryfallBulkDataMeta) -> Self { Self { client, bulk_data } }

    pub fn meta(&self) -> &ScryfallBulkDataMeta { &self.bulk_data }

    pub async fn data(self) -> anyhow::Result<Vec<ScryfallCard>> {
        self.client
            .request_client
            .get(&self.bulk_data.download_uri)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<ScryfallCard>>()
            .await
            .map_err(anyhow::Error::new)
    }

    pub async fn stream_to_file(self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let mut stream = self
            .client
            .request_client
            .get(&self.bulk_data.download_uri)
            .send()
            .await?
            .error_for_status()?
            .bytes_stream();

        let mut file = File::create(path).await?;
        while let Some(chunk) = stream.try_next().await? {
            file.write_all(&chunk).await?;
        }

        file.flush().await?;

        Ok(())
    }
}
