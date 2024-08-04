use std::f32::consts::PI;

use bevy::prelude::*;
use hari::physics::{components::Velocity, PhysicsMovementBundle};
use rand::prelude::*;

use super::components::{Player, Seagull};

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
    let player_position = Vec3::new(0., -60., 100.);

    // Player
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(player_position.clone()),
            texture: boat_texture.clone(),
            ..default()
        },
        PhysicsMovementBundle::new(player_position.clone(), Vec3::new(0., 0., 0.)),
        Player,
    ));
}

/// Handle keyboard input to move the player.
pub fn handle_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
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

pub fn spawn_seagull(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sea_gull_query: Query<&Seagull>,
) {
    let mut rng = rand::thread_rng();
    let starting_x = rng.gen::<f32>() * 1700. - 850.;

    let starting_position = Vec3::new(starting_x, 300., 1.);

    if sea_gull_query.is_empty() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(starting_position.clone()),
                texture: asset_server.load("1920x1080/gull_1_64x50.png"),
                ..default()
            },
            Seagull,
            PhysicsMovementBundle::new(starting_position, Vec3::new(0., -280., 0.)),
        ));
    }
}

pub fn despawn_seagull(
    mut commands: Commands,
    sea_gull_query: Query<(Entity, &Transform), With<Seagull>>,
) {
    let bottom_limit = -1080. / 2. + 300.;

    for (entity, &transform) in sea_gull_query.iter() {
        if transform.translation.y < bottom_limit {
            commands.entity(entity).despawn();
        }
    }
}
