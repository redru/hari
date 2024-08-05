use bevy::prelude::*;
use components::{SeagullCounter, SeagullSpawnTimer};
use hari::physics::{PhysicsPlugin, PhysicsSet};

mod components;
mod systems;

/// Since Bevy's default 2D camera setup is scaled such that
/// one unit is one pixel, you can think of this as
/// "How many pixels per second should the player move?"
pub const PLAYER_SPEED: f32 = 500.0;
pub const MAX_SEAGULLS: i32 = 5;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin)
            .insert_resource(SeagullSpawnTimer(Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            )))
            .insert_resource(SeagullCounter(0))
            .add_systems(Startup, systems::setup_system)
            .add_systems(Update, systems::handle_input_system.after(PhysicsSet))
            .add_systems(Update, systems::despawn_seagull)
            .add_systems(FixedUpdate, systems::spawn_seagull);
    }
}
