use std::f32::consts::PI;

use bevy::prelude::*;
use hari::physics::components::Velocity;

use crate::game::math_utils::atan2;

use super::components::Shark;

const SHARK_HORIZONTAL_DECELERATION: f32 = 8.;

pub fn shark_movement_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Shark>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        match velocity.x {
            x if x > SHARK_HORIZONTAL_DECELERATION => {
                velocity.x -= SHARK_HORIZONTAL_DECELERATION * fixed_time.delta_seconds()
            }
            x if x < SHARK_HORIZONTAL_DECELERATION => {
                velocity.x += SHARK_HORIZONTAL_DECELERATION * fixed_time.delta_seconds()
            }
            _ => velocity.x = 0.,
        }

        // let previous_rot_y = transform.rotation.y;
        let direction = velocity.normalize_or_zero();

        match direction.x {
            x if x < 0. => {
                transform.rotation = Quat::default();
                transform.rotate_y(PI);
                transform.rotate_z(atan2(direction) + PI);
            }
            _ => {
                transform.rotation = Quat::from_rotation_z(atan2(direction));
            }
        }
    }
}

pub fn shark_despawn_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Shark>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.y <= -200. {
            commands.entity(entity).despawn();
        }
    }
}
