use bevy::prelude::*;
use components::Shark;
use hari::physics::{components::Gravity, PhysicsMovementBundle, PhysicsSet};
use systems::{shark_despawn_system, shark_movement_system};

mod components;
mod systems;

pub struct HariSharkPlugin;

impl Plugin for HariSharkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shark_despawn_system)
            .add_systems(FixedUpdate, shark_movement_system.after(PhysicsSet));
    }
}

pub fn new_shark_bundle(
    asset_server: Res<AssetServer>,
    position: Vec3,
    velocity: Vec3,
) -> impl Bundle {
    (
        SpriteBundle {
            transform: Transform::from_translation(position),
            texture: asset_server.load("1920x1080/shark_87x50.png"),
            ..default()
        },
        PhysicsMovementBundle::new(position, velocity),
        Gravity::default(),
        Shark,
    )
}
