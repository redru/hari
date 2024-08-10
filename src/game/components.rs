use bevy::prelude::*;
use hari::physics::{components::RectangleCollider, PhysicsMovementBundle};

const SEAGULL_FALL_SPEED: f32 = 280.;
const SEAGULL_WIDTH: f32 = 64.;
const SEAGULL_HEIGHT: f32 = 50.;

const SCORE_GIZMO_RUNNER_DEST: Vec2 = Vec2::new(-940., 500.);
const SCORE_GIRZMO_RUNNER_SPEED: f32 = 2000.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Seagull;

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct DestinationAndDestroy(pub Vec2);

#[derive(Component)]
pub struct ScoreRunner;

#[derive(Resource)]
pub struct SeagullCounter(pub i32);

#[derive(Resource)]
pub struct SeagullSpawnTimer(pub Timer);

#[derive(Resource)]
pub struct CurrentScore(pub i32);

#[derive(Event)]
pub struct SeagullCaught {
    pub score: i32,
}

impl SeagullCaught {
    pub fn new(score: i32) -> Self {
        Self { score }
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

pub fn spawn_seagull_score_gizmo_runner(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec2,
) {
    let velocity =
        (SCORE_GIZMO_RUNNER_DEST - position).normalize_or_zero() * SCORE_GIRZMO_RUNNER_SPEED;

    let position_v3 = Vec3::new(position.x, position.y, 200.);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(position_v3),
            texture: asset_server.load("1920x1080/gull_gizmo_32x25.png"),
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.5),
                ..default()
            },
            ..default()
        },
        PhysicsMovementBundle::new(position_v3, Vec3::new(velocity.x, velocity.y, 0.)),
        DestinationAndDestroy(SCORE_GIZMO_RUNNER_DEST),
        ScoreRunner,
    ));
}
