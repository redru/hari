use std::f32::consts::PI;

use bevy::prelude::*;
use hari::physics::{components::Velocity, PhysicsMovementBundle};

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Background
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        texture: asset_server.load("1920x1080/background.png"),
        ..default()
    });

    let boat_texture = asset_server.load("1920x1080/boat.png");
    let player_position = Vec3::new(0., -60., 1.);

    // Player
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(player_position.clone()),
            texture: boat_texture.clone(),
            ..default()
        },
        PhysicsMovementBundle::new(player_position.clone(), Vec3::new(0., 0., 0.)),
        super::components::Player,
    ));

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-600., 300., 1.),
        texture: asset_server.load("1920x1080/gull_1_64x50.png"),
        ..default()
    });
}

/// Handle keyboard input to move the player.
pub fn handle_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<super::components::Player>>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        velocity.0 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x -= 1.0;
            transform.rotation = Quat::from_rotation_y(PI);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x += 1.0;
            transform.rotation = Quat::default();
        }

        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        velocity.0 = velocity.normalize_or_zero() * super::PLAYER_SPEED;
    }
}
