use bevy::prelude::*;
use camera::HariCameraPlugin;
use components::{CurrentScore, SeagullCaught};
use hari::physics::{PhysicsPlugin, PhysicsSet};
use player::HariPlayerPlugin;
use seagull::SeagullPlugin;
use ui::HariUIPlugin;

mod camera;
mod components;
mod math_utils;
mod player;
mod seagull;
mod systems;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin)
            .add_plugins(HariCameraPlugin)
            .add_plugins(HariPlayerPlugin)
            .add_plugins(SeagullPlugin)
            .add_plugins(HariUIPlugin)
            .insert_resource(CurrentScore(0))
            .add_event::<SeagullCaught>()
            .add_systems(Startup, systems::setup_system)
            .add_systems(Update, systems::score_runner_system)
            .add_systems(
                FixedUpdate,
                systems::check_player_collision.after(PhysicsSet),
            );
    }
}
