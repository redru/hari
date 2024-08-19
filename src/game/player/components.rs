use bevy::prelude::*;

use super::PLAYER_OSCILLATION_SECONDS;

pub enum Movement {
    None,
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub movement: Movement,
    pub oscillation_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement: Movement::None,
            oscillation_timer: Timer::from_seconds(
                PLAYER_OSCILLATION_SECONDS,
                TimerMode::Repeating,
            ),
        }
    }
}
