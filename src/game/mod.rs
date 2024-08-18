use bevy::prelude::*;
use camera::HariCameraPlugin;
use components::{CurrentScore, SeagullCaught, SeagullCounter, SeagullSpawnTimer};
use hari::physics::{PhysicsPlugin, PhysicsSet};
use player::HariPlayerPlugin;

mod camera;
mod components;
mod math_utils;
mod player;
mod systems;

pub const MAX_SEAGULLS: i32 = 5;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin)
            .add_plugins(HariCameraPlugin)
            .add_plugins(HariPlayerPlugin)
            .insert_resource(SeagullSpawnTimer(Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            )))
            .insert_resource(SeagullCounter(0))
            .insert_resource(CurrentScore(0))
            .add_event::<SeagullCaught>()
            .add_systems(Startup, (systems::setup_system, systems::setup_ui_system))
            .add_systems(Update, systems::score_runner_system)
            .add_systems(
                FixedUpdate,
                (
                    systems::spawn_seagull_system,
                    systems::despawn_seagull_system,
                    systems::update_score_system,
                ),
            )
            .add_systems(
                FixedUpdate,
                systems::check_player_collision.after(PhysicsSet),
            );
    }
}
