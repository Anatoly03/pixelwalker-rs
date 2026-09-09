use crate::PWCollection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct World {
    pub created: String,
    pub description: String,
    pub id: String,
    #[serde(rename = "maxPlayers")]
    pub max_players: usize,
    pub owner: String,
    pub plays: usize,
    #[serde(rename = "publishedWorld")]
    pub published_world: String,
    pub title: String,
    pub updated: String,
    pub visibility: String,
    pub woots: usize,
    #[serde(rename = "worldData")]
    pub world_data: String,
}

impl PWCollection for World {
    const COLLECTION_NAME: &'static str = "worlds";
}
