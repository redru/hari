use bevy::prelude::*;
use systems::camera_startup_system;

pub mod systems;

pub struct HariCameraPlugin;

impl Plugin for HariCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_startup_system);
    }
}
