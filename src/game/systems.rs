use bevy::prelude::*;
use hari::physics::{
    collisions::{rectangles_collision_axis_aligned, CollisionRectangle},
    components::{RectangleCollider, Velocity},
};

use super::{
    components::{
        spawn_seagull_score_gizmo_runner, DestinationAndDestroy, ScoreRunner, SeagullCaught,
    },
    math_utils::vec2_faces_point,
    player::{components::Player, PLAYER_COLLIDER_OFFSET},
    seagull::components::{Seagull, SeagullCounter},
};

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Background
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., 60., 0.),
        texture: asset_server.load("1920x1080/background.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., -305., 200.),
        texture: asset_server.load("1920x1080/background_front.png"),
        ..default()
    });
}

pub fn check_player_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut seagull_counter: ResMut<SeagullCounter>,
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
            seagull_counter.0 -= 1;
            commands.entity(seagull_entity).despawn();
            spawn_seagull_score_gizmo_runner(&mut commands, &asset_server, seagull_impact_position);
        }
    }
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
