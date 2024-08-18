use bevy::prelude::*;
use hari::physics::PhysicsSet;
use systems::{handle_input_system, player_startup_system};

pub mod components;
pub mod systems;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_COLLIDER_WIDTH: f32 = 220.0;
pub const PLAYER_COLLIDER_HEIGHT: f32 = 50.0;
pub const PLAYER_COLLIDER_OFFSET: Vec2 = Vec2::new(0.0, -120.0);

pub struct HariPlayerPlugin;

impl Plugin for HariPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_startup_system)
            .add_systems(Update, handle_input_system.after(PhysicsSet));
    }
}
