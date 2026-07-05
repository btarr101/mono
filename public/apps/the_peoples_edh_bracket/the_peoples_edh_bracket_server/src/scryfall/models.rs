use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct ScryfallList<T> {
    #[serde_inline_default(false)]
    pub has_more: bool,
    pub data: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct ScryfallCard {
    pub oracle_id: Option<uuid::Uuid>,
    pub name: String,
    pub flavor_name: Option<String>,
    pub printed_name: Option<String>,
    pub image_uris: Option<ScryfallImageUris>,
    pub legalities: ScryfallLegalities,
    pub card_faces: Option<Vec<ScryfallCardFace>>,
}

#[derive(Deserialize, Debug)]
pub struct ScryfallCardFace {
    pub image_uris: Option<ScryfallImageUris>,
}

impl ScryfallCard {
    pub fn mediumest_image_uri(&self) -> Option<&str> {
        self.image_uris
            .as_ref()
            .and_then(|image_uris| image_uris.mediumest())
            .or(self.card_faces.as_ref().and_then(|faces| {
                faces
                    .iter()
                    .find_map(|face| face.image_uris.as_ref().and_then(|image_uris| image_uris.mediumest()))
            }))
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ScryfallImageUris {
    png: Option<String>,
    large: Option<String>,
    normal: Option<String>,
    small: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScryfallLegality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ScryfallLegalities {
    pub commander: ScryfallLegality,
}

impl ScryfallImageUris {
    pub fn mediumest(&self) -> Option<&str> {
        self.normal
            .as_deref()
            .or(self.small.as_deref())
            .or(self.large.as_deref())
            .or(self.png.as_deref())
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ScryfallBulkDataMeta {
    #[serde(rename = "type")]
    pub ty: String,
    pub download_uri: String,
    pub size: usize,
}
