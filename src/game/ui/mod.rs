use bevy::prelude::*;
use systems::{ui_startup_system, update_score_system};

pub mod systems;

pub struct HariUIPlugin;

impl Plugin for HariUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_startup_system)
            .add_systems(Update, update_score_system);
    }
}
