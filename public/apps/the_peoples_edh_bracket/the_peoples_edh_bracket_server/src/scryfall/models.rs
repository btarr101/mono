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
    pub oracle_id: uuid::Uuid,
    pub name: String,
    pub image_uris: Option<ScryfallImageUris>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ScryfallImageUris {
    png: Option<String>,
    large: Option<String>,
    normal: Option<String>,
    small: Option<String>,
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
