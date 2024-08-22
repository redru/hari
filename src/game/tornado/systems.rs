use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::game::math_utils::lerp_f32;

use super::components::{Tornado, TornadoState, TornadoTimer, TORNADO_ENTERING_DURATION};

const TORNADO_SPAWN_HEIGHT: f32 = 700.;

pub fn spawn_tornado(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fixed_time: Res<Time<Fixed>>,
    mut tornado_timer: ResMut<TornadoTimer>,
) {
    if tornado_timer.0.tick(fixed_time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let starting_x = rng.gen_range(-850.0..=850.0);

        let tornado_position = Vec3::new(starting_x, TORNADO_SPAWN_HEIGHT, 99.);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(tornado_position),
                texture: asset_server.load("1920x1080/tornado_217x208.png"),
                ..default()
            },
            Tornado::new(),
        ));
    }
}

pub fn tornado_lifecycle_system(
    mut commands: Commands,
    fixed_time: Res<Time<Fixed>>,
    mut tornado_timer: ResMut<TornadoTimer>,
    mut query: Query<(Entity, &mut Tornado)>,
) {
    if query.iter().count() == 0 {
        return;
    }

    let (entity, mut tornado) = query.single_mut();

    tornado.timer.tick(fixed_time.delta());

    match tornado.state {
        super::components::TornadoState::Entering => {
            if tornado.timer.just_finished() {
                tornado.state = TornadoState::Executing;
                tornado.timer.reset();
                tornado.timer.set_duration(Duration::from_secs(10));
            }
        }
        super::components::TornadoState::Executing => {
            if tornado.timer.just_finished() {
                tornado.state = TornadoState::Exiting;
                tornado.timer.reset();
                tornado.timer.set_duration(Duration::from_secs(5));
            }
        }
        super::components::TornadoState::Exiting => {
            if tornado.timer.just_finished() {
                commands.entity(entity).despawn();
                tornado_timer.reset();
            }
        }
    }
}

pub fn tornado_executor_system(mut query: Query<(&mut Transform, &Tornado)>) {
    if query.iter().count() == 0 {
        return;
    }

    let (mut transform, tornado) = query.single_mut();

    match tornado.state {
        TornadoState::Entering => {
            transform.translation.y = lerp_f32(
                TORNADO_SPAWN_HEIGHT,
                0.,
                tornado.timer.elapsed_secs() / TORNADO_ENTERING_DURATION,
            );
        }
        TornadoState::Executing => {}
        TornadoState::Exiting => {}
    }
}
