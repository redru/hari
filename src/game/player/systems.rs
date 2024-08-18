use std::f32::consts::PI;

use bevy::{color::palettes::css::GREEN, prelude::*, sprite::MaterialMesh2dBundle};
use hari::physics::{
    components::{RectangleCollider, Velocity},
    PhysicsMovementBundle,
};

use super::{
    components::Player, PLAYER_COLLIDER_HEIGHT, PLAYER_COLLIDER_OFFSET, PLAYER_COLLIDER_WIDTH,
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
            PhysicsMovementBundle::new(player_position.clone(), Vec3::new(0., 0., 0.)),
            Player,
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
