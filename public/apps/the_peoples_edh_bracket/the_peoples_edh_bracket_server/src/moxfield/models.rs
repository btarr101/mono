use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MoxfieldDeck {
    pub id: String,
    pub name: String,
    #[serde(rename = "exportId")]
    pub export_id: String,
}
