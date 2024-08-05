use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use hari::physics::{
    components::{Collider, Velocity},
    PhysicsMovementBundle,
};
use rand::prelude::*;

use super::{
    components::{Player, Seagull, SeagullCounter, SeagullSpawnTimer},
    MAX_SEAGULLS,
};

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
        Collider::default(),
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
    time: Res<Time<Fixed>>,
    mut seagull_counter: ResMut<SeagullCounter>,
    mut seagull_spawn_timer: ResMut<SeagullSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if seagull_counter.0 < MAX_SEAGULLS {
        if seagull_spawn_timer.0.tick(time.delta()).just_finished() {
            let mut rng = rand::thread_rng();
            let starting_x = rng.gen::<f32>() * 1700. - 850.;

            let starting_position = Vec3::new(starting_x, 600., 1.);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(starting_position.clone()),
                    texture: asset_server.load("1920x1080/gull_1_64x50.png"),
                    ..default()
                },
                Seagull,
                PhysicsMovementBundle::new(starting_position, Vec3::new(0., -280., 0.)),
                Collider::default(),
            ));

            seagull_counter.0 += 1;

            let mut rng = rand::thread_rng();
            let new_spawn_duration = rng.gen_range(100..=1300);
            seagull_spawn_timer
                .0
                .set_duration(Duration::from_millis(new_spawn_duration));
        }
    }
}

pub fn despawn_seagull(
    mut commands: Commands,
    mut seagull_counter: ResMut<SeagullCounter>,
    sea_gull_query: Query<(Entity, &Transform), With<Seagull>>,
) {
    let bottom_limit = -1080. / 2. + 300.;

    for (entity, &transform) in sea_gull_query.iter() {
        if transform.translation.y < bottom_limit {
            commands.entity(entity).despawn();
            seagull_counter.0 -= 1;
        }
    }
}
