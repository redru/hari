use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Seagull;

#[derive(Resource)]
pub struct SeagullCounter(pub i32);

#[derive(Resource)]
pub struct SeagullSpawnTimer(pub Timer);

#[derive(Resource)]
pub struct CurrentScore(pub i32);

#[derive(Event)]
pub struct SeagullCaught(pub Entity, pub i32);
