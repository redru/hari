use bevy::prelude::*;

pub fn vec2_faces_point(direction: Vec2, position: Vec2, destination: Vec2) -> bool {
    let direction_to_dest = (destination - position).normalize_or_zero();

    if direction_to_dest == Vec2::ZERO {
        return true;
    }

    direction.dot(direction_to_dest) > 0.9
}

pub fn lerp_f32(start_value: f32, end_value: f32, pct: f32) -> f32 {
    start_value + (end_value - start_value) * pct
}

pub fn easy_in(t: f32) -> f32 {
    t * t
}
