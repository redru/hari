use bevy::prelude::*;
use hari::physics::{components::RectangleCollider, PhysicsMovementBundle};

const SEAGULL_FALL_SPEED: f32 = 280.;
const SEAGULL_WIDTH: f32 = 64.;
const SEAGULL_HEIGHT: f32 = 50.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Seagull;

#[derive(Component)]
pub struct Score;

#[derive(Resource)]
pub struct SeagullCounter(pub i32);

#[derive(Resource)]
pub struct SeagullSpawnTimer(pub Timer);

#[derive(Resource)]
pub struct CurrentScore(pub i32);

#[derive(Event)]
pub struct SeagullCaught {
    pub entity: Entity,
    pub score: i32,
}

impl SeagullCaught {
    pub fn new(entity: Entity, score: i32) -> Self {
        Self { entity, score }
    }
}

pub fn spawn_seagull(commands: &mut Commands, asset_server: &AssetServer, translation: Vec3) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(translation.clone()),
            texture: asset_server.load("1920x1080/gull_1_64x50.png"),
            ..default()
        },
        Seagull,
        PhysicsMovementBundle::new(translation.clone(), Vec3::new(0., -SEAGULL_FALL_SPEED, 0.)),
        RectangleCollider::new(true, SEAGULL_WIDTH, SEAGULL_HEIGHT),
    ));
}
