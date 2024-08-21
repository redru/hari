use std::f32::consts::PI;

use bevy::prelude::*;
use hari::physics::{
    components::{Gravity, RectangleCollider, Velocity},
    PhysicsMovementBundle,
};

use crate::game::math_utils::{lerp_ease_in_out_quad, lerp_f32};

use super::{
    components::{Movement, Player},
    PLAYER_COLLIDER_HEIGHT, PLAYER_COLLIDER_WIDTH, PLAYER_OSCILLATION_MAX,
    PLAYER_OSCILLATION_SECONDS, PLAYER_SPEED,
};

pub fn player_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_position = Vec3::new(0., 0., 100.);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(player_position.clone()),
            texture: asset_server.load("1920x1080/boat.png"),
            ..default()
        },
        Player::default(),
        PhysicsMovementBundle::new(player_position.clone(), Vec3::new(0., 0., 0.)),
        Gravity::default(),
        RectangleCollider::new(true, PLAYER_COLLIDER_WIDTH, PLAYER_COLLIDER_HEIGHT),
    ));
    // .with_children(|parent| {
    //     parent.spawn(MaterialMesh2dBundle {
    //         mesh: meshes
    //             .add(Rectangle::new(
    //                 PLAYER_COLLIDER_WIDTH,
    //                 PLAYER_COLLIDER_HEIGHT,
    //             ))
    //             .into(),
    //         transform: Transform::from_translation(Vec3::new(
    //             PLAYER_COLLIDER_OFFSET.x,
    //             PLAYER_COLLIDER_OFFSET.y,
    //             100.,
    //         )),
    //         material: materials.add(Color::from(GREEN)),
    //         ..default()
    //     });
    // });
}

/// Handle keyboard input to move the player.
pub fn handle_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    for mut player in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.movement = Movement::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.movement = Movement::Right;
        }
        if !keyboard_input.pressed(KeyCode::KeyA) && !keyboard_input.pressed(KeyCode::KeyD) {
            player.movement = Movement::None;
        }
    }
}

pub fn player_movement_system(mut query: Query<(&Player, &mut Transform, &mut Velocity)>) {
    let (player, mut transform, mut velocity) = query.single_mut();
    let mut velocity_increase = 0.0;
    let previous_rotation = transform.rotation;
    transform.rotation = Quat::default();
    transform.rotate_z(previous_rotation.to_euler(EulerRot::YXZ).2);

    match player.movement {
        Movement::None => transform.rotate_y(previous_rotation.to_euler(EulerRot::YXZ).0),
        Movement::Left => {
            transform.rotate_y(PI);
            velocity_increase = -20.0;
        }
        Movement::Right => {
            velocity_increase = 20.0;
        }
    };

    if velocity_increase == 0.0 {
        // If no input press, change velocity until reaching 0 using lerp
        velocity.x = lerp_f32(velocity.x, 0.0, 0.05);
    } else {
        velocity.x += velocity_increase;

        // Don't exit the PLAYER_SPEED boundaries
        if velocity.x > PLAYER_SPEED {
            velocity.x = PLAYER_SPEED;
        } else if velocity.x < -PLAYER_SPEED {
            velocity.x = -PLAYER_SPEED;
        }
    }
}

const WATER_FORCE_LEVEL: f32 = 0.0;
const WATER_FORCE: f32 = 4.9;

pub fn handle_player_floating_system(
    mut player_query: Query<(&Transform, &mut Velocity), With<Player>>,
) {
    let (transform, mut velocity) = player_query.single_mut();

    if transform.translation.y < WATER_FORCE_LEVEL {
        let sea_level_distance = transform.translation.y.abs();
        velocity.y += WATER_FORCE * sea_level_distance / 8.0;
    }

    // Limit vertical velocity to avoid the ship jumping when emerging
    if velocity.y > 40.0 {
        velocity.y = 40.0;
    }

    if velocity.y < -60.0 {
        velocity.y = -60.0;
    }
}

pub fn oscillate_player(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();
    player.oscillation_timer.tick(fixed_time.delta());

    let rotation_z = lerp_ease_in_out_quad(
        -PLAYER_OSCILLATION_MAX,
        PLAYER_OSCILLATION_MAX,
        (PLAYER_OSCILLATION_SECONDS - player.oscillation_timer.elapsed_secs())
            / PLAYER_OSCILLATION_SECONDS,
    );

    const ROTATION_VELOCITY: f32 = 30.0;

    let previous_rotation = transform.rotation;
    transform.rotation = Quat::default();
    transform.rotate_y(previous_rotation.to_euler(EulerRot::YXZ).0);
    transform.rotate_z(rotation_z * fixed_time.delta_seconds() * ROTATION_VELOCITY);
}
