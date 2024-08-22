use bevy::prelude::*;
use components::TornadoTimer;
use systems::{
    spawn_tornado, tornado_executor_system, tornado_lifecycle_system, tornado_shot_system,
};

pub mod components;
pub mod systems;

pub struct HariTornadoPlugin;

const TORNATO_COLDOWN: f32 = 2.;

impl Plugin for HariTornadoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TornadoTimer(Timer::from_seconds(
            TORNATO_COLDOWN,
            TimerMode::Once,
        )))
        .add_systems(Update, tornado_executor_system)
        .add_systems(
            FixedUpdate,
            (spawn_tornado, tornado_lifecycle_system, tornado_shot_system),
        );
    }
}
