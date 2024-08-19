use std::f32::consts::PI;

use bevy::prelude::*;
use hari::physics::{
    components::{Gravity, RectangleCollider, Velocity},
    PhysicsMovementBundle,
};

use crate::game::math_utils::lerp_f32;

use super::{
    components::{Movement, Player},
    PLAYER_COLLIDER_HEIGHT, PLAYER_COLLIDER_WIDTH, PLAYER_SPEED,
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
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.movement = Movement::Left;
            transform.rotation = Quat::from_rotation_y(PI);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.movement = Movement::Right;
            transform.rotation = Quat::default();
        }
        if !keyboard_input.pressed(KeyCode::KeyA) && !keyboard_input.pressed(KeyCode::KeyD) {
            player.movement = Movement::None;
        }
    }
}

pub fn player_movement_system(mut query: Query<(&Player, &mut Velocity)>) {
    let (player, mut velocity) = query.single_mut();

    let velocity_increase = match player.movement {
        Movement::None => 0.0,
        Movement::Left => -1.0,
        Movement::Right => 1.0,
    };

    if velocity_increase == 0.0 {
        // If no input press, change velocity using lerp 0
        velocity.x = lerp_f32(velocity.x, 0.0, 0.002);
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

    if velocity.y > 40.0 {
        velocity.y = 40.0;
    }

    if velocity.y < -60.0 {
        velocity.y = -60.0;
    }
}
