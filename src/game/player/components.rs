use bevy::prelude::*;

pub enum Movement {
    None,
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub movement: Movement,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement: Movement::None,
        }
    }
}
