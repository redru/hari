use std::f32::consts::PI;

use bevy::prelude::*;
use hari::physics::PhysicsSet;
use systems::{
    handle_input_system, handle_player_floating_system, oscillate_player, player_movement_system,
    player_startup_system,
};

pub mod components;
pub mod systems;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_COLLIDER_WIDTH: f32 = 220.0;
pub const PLAYER_COLLIDER_HEIGHT: f32 = 18.0;
pub const PLAYER_COLLIDER_OFFSET: Vec2 = Vec2::new(0.0, -100.0);
pub const PLAYER_OSCILLATION_SECONDS: f32 = 4.0;
pub const PLAYER_OSCILLATION_MAX: f32 = PI / 30.0;

pub struct HariPlayerPlugin;

impl Plugin for HariPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_startup_system)
            .add_systems(Update, handle_input_system.after(PhysicsSet))
            .add_systems(
                FixedUpdate,
                (
                    player_movement_system,
                    handle_player_floating_system,
                    oscillate_player,
                )
                    .after(PhysicsSet),
            );
    }
}
