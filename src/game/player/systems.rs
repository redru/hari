use std::f32::consts::PI;

use bevy::{color::palettes::css::GREEN, prelude::*, sprite::MaterialMesh2dBundle};
use hari::physics::{
    components::{Gravity, RectangleCollider, Velocity},
    PhysicsMovementBundle,
};

use crate::game::math_utils::lerp_f32;

use super::{
    components::Player, PLAYER_COLLIDER_HEIGHT, PLAYER_COLLIDER_OFFSET, PLAYER_COLLIDER_WIDTH,
    PLAYER_SPEED,
};

pub fn player_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_position = Vec3::new(0., 0., 100.);

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(player_position.clone()),
                texture: asset_server.load("1920x1080/boat.png"),
                ..default()
            },
            Player,
            PhysicsMovementBundle::new(player_position.clone(), Vec3::new(0., 0., 0.)),
            Gravity::new(0.0),
            RectangleCollider::new(true, PLAYER_COLLIDER_WIDTH, PLAYER_COLLIDER_HEIGHT),
        ))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Rectangle::new(
                        PLAYER_COLLIDER_WIDTH,
                        PLAYER_COLLIDER_HEIGHT,
                    ))
                    .into(),
                transform: Transform::from_translation(Vec3::new(
                    PLAYER_COLLIDER_OFFSET.x,
                    PLAYER_COLLIDER_OFFSET.y,
                    100.,
                )),
                material: materials.add(Color::from(GREEN)),
                ..default()
            });
        });
}

/// Handle keyboard input to move the player.
pub fn handle_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let mut velocity_increase = 0.0;

        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity_increase = -1.0;
            transform.rotation = Quat::from_rotation_y(PI);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity_increase = 1.0;
            transform.rotation = Quat::default();
        }

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
}
