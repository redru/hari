use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use super::{
    components::{spawn_seagull, Seagull, SeagullCounter, SeagullSpawnTimer},
    MAX_SEAGULLS,
};

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

            let starting_position = Vec3::new(starting_x, 660., 1.);

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
