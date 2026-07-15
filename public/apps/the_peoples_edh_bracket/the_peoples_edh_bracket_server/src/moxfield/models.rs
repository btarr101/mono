use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MoxfieldDeck {
    pub id: String,
    pub name: String,
    #[serde(rename = "exportId")]
    pub export_id: String,
    pub boards: MoxfieldBoards,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MoxfieldBoards {
    pub commanders: MoxfieldBoard,
    pub mainboard: MoxfieldBoard,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MoxfieldBoard {
    pub cards: HashMap<String, MoxfieldBoardEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MoxfieldBoardEntry {
    pub quantity: usize,
    pub card: MoxfieldCard,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MoxfieldCard {
    pub name: String,
}
