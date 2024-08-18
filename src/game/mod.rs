use bevy::prelude::*;
use camera::HariCameraPlugin;
use components::{CurrentScore, SeagullCaught};
use hari::physics::{PhysicsPlugin, PhysicsSet};
use player::HariPlayerPlugin;
use seagull::SeagullPlugin;

mod camera;
mod components;
mod math_utils;
mod player;
mod seagull;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin)
            .add_plugins(HariCameraPlugin)
            .add_plugins(HariPlayerPlugin)
            .add_plugins(SeagullPlugin)
            .insert_resource(CurrentScore(0))
            .add_event::<SeagullCaught>()
            .add_systems(Startup, (systems::setup_system, systems::setup_ui_system))
            .add_systems(Update, systems::score_runner_system)
            .add_systems(FixedUpdate, systems::update_score_system)
            .add_systems(
                FixedUpdate,
                systems::check_player_collision.after(PhysicsSet),
            );
    }
}
