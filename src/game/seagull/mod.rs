use bevy::prelude::*;
use components::{SeagullCounter, SeagullSpawnTimer};

pub mod components;
pub mod systems;

pub const MAX_SEAGULLS: i32 = 5;

pub struct SeagullPlugin;

impl Plugin for SeagullPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SeagullSpawnTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .insert_resource(SeagullCounter(0))
        .add_systems(
            FixedUpdate,
            (
                systems::spawn_seagull_system,
                systems::despawn_seagull_system,
            ),
        );
    }
}
