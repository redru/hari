use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use hari::physics::{
    collisions::{rectangles_collision_axis_aligned, CollisionRectangle},
    components::{RectangleCollider, Velocity},
    PhysicsMovementBundle,
};
use rand::prelude::*;

use super::{
    components::{
        spawn_seagull, spawn_seagull_score_gizmo_runner, CurrentScore, DestinationAndDestroy,
        Player, PlayerAnimation, Score, ScoreRunner, Seagull, SeagullCaught, SeagullCounter,
        SeagullSpawnTimer, Shark,
    },
    math_utils::vec2_faces_point,
    MAX_SEAGULLS, PLAYER_COLLIDER_HEIGHT, PLAYER_COLLIDER_OFFSET, PLAYER_COLLIDER_WIDTH,
};

pub fn setup_test_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., -200., 101.),
            texture: asset_server.load("1920x1080/shark_200x200.png"),
            ..default()
        },
        Shark::new(4000),
    ));
}

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Background
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        texture: asset_server.load("1920x1080/background.png"),
        ..default()
    });

    // Background front
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., -365., 200.),
        texture: asset_server.load("1920x1080/background_front.png"),
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
        RectangleCollider::new(true, PLAYER_COLLIDER_WIDTH, PLAYER_COLLIDER_HEIGHT),
        PlayerAnimation::new(player_position.y, player_position.y - 20.),
    ));
}

pub fn setup_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_score: ResMut<CurrentScore>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(20.)),
                width: Val::Vw(100.0),
                height: Val::Px(60.0),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(32.),
                        height: Val::Px(25.),
                        ..default()
                    },
                    ..default()
                },
                UiImage::new(asset_server.load("1920x1080/gull_gizmo_32x25.png")),
            ));

            parent
                .spawn(NodeBundle {
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("{}", current_score.0),
                            TextStyle {
                                font: asset_server
                                    .load("1920x1080/Inconsolata-VariableFont_wdth,wght.ttf"),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        ),
                        Score,
                    ));
                });
        });
}

/// Handle keyboard input to move the player.
pub fn handle_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        velocity.x = 0.;

        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x -= 1.0;
            transform.rotation = Quat::from_rotation_y(PI);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x += 1.0;
            transform.rotation = Quat::default();
        }

        velocity.x = velocity.x * super::PLAYER_SPEED;
    }
}

pub fn spawn_seagull_system(
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

            spawn_seagull(&mut commands, &asset_server, starting_position);

            seagull_counter.0 += 1;

            let mut rng = rand::thread_rng();
            let new_spawn_duration = rng.gen_range(100..=1300);
            seagull_spawn_timer
                .0
                .set_duration(Duration::from_millis(new_spawn_duration));
        }
    }
}

pub fn despawn_seagull_system(
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_collider_query: Query<(&Transform, &RectangleCollider), With<Player>>,
    mut seagull_collider_query: Query<(Entity, &Transform, &RectangleCollider), With<Seagull>>,
) {
    let (player_transform, player_rectangle_collider) = player_collider_query.single();
    let player_collision_rect = CollisionRectangle::from_translation(
        player_transform.translation.xy(),
        player_rectangle_collider.width,
        player_rectangle_collider.height,
    )
    .with_offset(PLAYER_COLLIDER_OFFSET);

    for (seagull_entity, seagull_transform, seagull_rectangle_collider) in
        seagull_collider_query.iter_mut()
    {
        if !seagull_rectangle_collider.enabled {
            continue;
        }

        let seagull_impact_position = seagull_transform.translation.xy();

        let seagull_collision_rect = CollisionRectangle::from_translation(
            seagull_impact_position,
            seagull_rectangle_collider.width,
            seagull_rectangle_collider.height,
        );

        if rectangles_collision_axis_aligned(player_collision_rect, seagull_collision_rect) {
            commands.entity(seagull_entity).despawn();
            spawn_seagull_score_gizmo_runner(&mut commands, &asset_server, seagull_impact_position);
        }
    }
}

pub fn update_score_system(
    mut seagull_counter: ResMut<SeagullCounter>,
    mut er_seagull_caught: EventReader<SeagullCaught>,
    mut current_score: ResMut<CurrentScore>,
    mut score_query: Query<&mut Text, With<Score>>,
) {
    for ev in er_seagull_caught.read() {
        seagull_counter.0 -= 1;
        current_score.0 += ev.score;
    }

    score_query.single_mut().sections.get_mut(0).unwrap().value = format!("{}", current_score.0);
}

pub fn score_runner_system(
    mut commands: Commands,
    mut ew_seagull_caught: EventWriter<SeagullCaught>,
    destination_and_destroy_query: Query<
        (Entity, &DestinationAndDestroy, &Transform, &Velocity),
        With<ScoreRunner>,
    >,
) {
    for (entity, destination_and_destroy, transform, velocity) in
        destination_and_destroy_query.iter()
    {
        if !vec2_faces_point(
            velocity.0.xy(),
            transform.translation.xy(),
            destination_and_destroy.0,
        ) {
            ew_seagull_caught.send(SeagullCaught::new(1));
            commands.entity(entity).despawn();
        }
    }
}

pub fn fixed_shark_system(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    mut shark_query: Query<(Entity, &mut Shark)>,
) {
    for (entity, mut shark) in shark_query.iter_mut() {
        if shark.alive_timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn animate_player(
    mut player_query: Query<(&Transform, &PlayerAnimation, &mut Velocity), With<Player>>,
) {
    const FLOAT_CHANGE_FACTOR: f32 = 1.1;
    const MAX_VEL: f32 = 20.;
    const MIN_VEL: f32 = 10.;

    let (player_transform, player_animation, mut player_velocity) =
        player_query.get_single_mut().unwrap();

    let current_y = player_transform.translation.y;

    if current_y >= player_animation.vertical_top_value {
        player_velocity.y = -1.;
    } else if current_y <= player_animation.vertical_bottom_value {
        player_velocity.y = 1.;
    }

    if player_velocity.y < 0. {
        if current_y > player_animation.halfway {
            // First half
            player_velocity.y *= FLOAT_CHANGE_FACTOR;
        } else {
            // Second half
            player_velocity.y /= FLOAT_CHANGE_FACTOR;
        }

        if player_velocity.y < -MAX_VEL {
            player_velocity.y = -MAX_VEL;
        } else if player_velocity.y > -MIN_VEL {
            player_velocity.y = -MIN_VEL;
        }
    } else if player_velocity.y > 0. {
        if current_y < player_animation.halfway {
            // First half
            player_velocity.y *= FLOAT_CHANGE_FACTOR;
        } else {
            // Second half
            player_velocity.y /= FLOAT_CHANGE_FACTOR;
        }

        if player_velocity.y > MAX_VEL {
            player_velocity.y = MAX_VEL;
        } else if player_velocity.y < MIN_VEL {
            player_velocity.y = MIN_VEL;
        }
    }
}
