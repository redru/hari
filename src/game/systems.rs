use std::{f32::consts::PI, time::Duration};

use bevy::{color::palettes::css::GREEN, prelude::*, sprite::MaterialMesh2dBundle};
use hari::physics::{
    collisions::{rectangles_collision_axis_aligned, CollisionRectangle},
    components::{RectangleCollider, Velocity},
    PhysicsMovementBundle,
};
use rand::prelude::*;

use super::{
    components::{CurrentScore, Player, Seagull, SeagullCaught, SeagullCounter, SeagullSpawnTimer},
    MAX_SEAGULLS, PLAYER_COLLIDER_HEIGHT, PLAYER_COLLIDER_OFFSET, PLAYER_COLLIDER_WIDTH,
};

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(player_position.clone()),
                texture: boat_texture.clone(),
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
            let starting_x = rng.gen_range(-850.0..=850.0);

            let starting_position = Vec3::new(starting_x, 600., 1.);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(starting_position.clone()),
                    texture: asset_server.load("1920x1080/gull_1_64x50.png"),
                    ..default()
                },
                Seagull,
                PhysicsMovementBundle::new(starting_position, Vec3::new(0., -280., 0.)),
                RectangleCollider::new(true, 64., 50.),
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

pub fn check_player_collision(
    mut ew_seagull_caught: EventWriter<SeagullCaught>,
    player_collider_query: Query<(&Transform, &RectangleCollider), With<Player>>,
    mut seagull_collider_query: Query<
        (Entity, &Transform, &mut RectangleCollider),
        (With<Seagull>, Without<Player>),
    >,
) {
    let (player_transform, player_rectangle_collider) = player_collider_query.single();
    let player_collision_rect = CollisionRectangle::from_translation(
        player_transform.translation.xy(),
        player_rectangle_collider.width,
        player_rectangle_collider.height,
    )
    .with_offset(PLAYER_COLLIDER_OFFSET);

    for (seagull_entity, seagull_transform, mut seagull_rectangle_collider) in
        seagull_collider_query.iter_mut()
    {
        if !seagull_rectangle_collider.enabled {
            continue;
        }

        let seagull_collision_rect = CollisionRectangle::from_translation(
            seagull_transform.translation.xy(),
            seagull_rectangle_collider.width,
            seagull_rectangle_collider.height,
        );

        let is_collision =
            rectangles_collision_axis_aligned(&player_collision_rect, &seagull_collision_rect);

        if is_collision {
            ew_seagull_caught.send(SeagullCaught(seagull_entity, 4));
            seagull_rectangle_collider.enabled = false;
        }
    }
}

pub fn update_score(
    mut commands: Commands,
    mut seagull_counter: ResMut<SeagullCounter>,
    mut er_seagull_caught: EventReader<SeagullCaught>,
    mut current_score: ResMut<CurrentScore>,
) {
    for ev in er_seagull_caught.read() {
        commands.entity(ev.0).despawn();
        seagull_counter.0 -= 1;
        current_score.0 += ev.1;

        println!("current score {}", current_score.0);
    }
}
