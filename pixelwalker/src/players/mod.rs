use std::{collections::HashMap, ops::Index};

#[derive(Default)]
pub struct PlayerManager {
    map: HashMap<usize, Player>,
}

impl Index<usize> for PlayerManager {
    type Output = Player;

    fn index(&self, index: usize) -> &Self::Output {
        &self.map[&index]
    }
}

#[derive(Default)]
pub struct Player {
    id: usize,
    username: String,
}

impl Player {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}
