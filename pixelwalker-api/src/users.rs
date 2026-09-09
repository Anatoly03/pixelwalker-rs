use crate::PWCollection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
    pub aura: String,
    #[serde(rename = "bonusEnergy")]
    pub bonus_energy: isize,
    pub created: String,
    #[serde(rename = "energyTime")]
    pub energy_time: String,
    pub id: String,
    #[serde(rename = "libraryCreatorPoints")]
    pub library_creator_points: isize,
    #[serde(rename = "libraryStars")]
    pub library_stars: isize,
    #[serde(rename = "libraryTrophies")]
    pub library_trophies: isize,
    #[serde(rename = "maxEnergy")]
    pub max_energy: isize,
    pub role: String,
    pub smiley: String,
    pub updated: String,
    pub username: String,
}

impl PWCollection for User {
    const COLLECTION_NAME: &'static str = "users";
}
