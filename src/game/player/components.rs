use std::time::Duration;

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
        let mut oscillation_timer =
            Timer::from_seconds(PLAYER_OSCILLATION_SECONDS, TimerMode::Repeating);

        oscillation_timer.set_elapsed(Duration::from_secs_f32(PLAYER_OSCILLATION_SECONDS / 2.0));

        Self {
            movement: Movement::None,
            oscillation_timer,
        }
    }
}
