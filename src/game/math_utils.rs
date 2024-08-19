use bevy::prelude::*;

pub fn vec2_faces_point(direction: Vec2, position: Vec2, destination: Vec2) -> bool {
    let direction_to_dest = (destination - position).normalize_or_zero();

    if direction_to_dest == Vec2::ZERO {
        return true;
    }

    direction.dot(direction_to_dest) > 0.9
}

pub fn lerp_f32(start_value: f32, end_value: f32, t: f32) -> f32 {
    start_value + (end_value - start_value) * t
}

// This must be fixed, it is wrong function
pub fn lerp_ease_in_out(start_value: f32, end_value: f32, t: f32) -> f32 {
    lerp_f32(start_value, end_value, spike(t))
}

pub fn flip(t: f32) -> f32 {
    1.0 - t
}

pub fn spike(t: f32) -> f32 {
    match t {
        0.0..0.5 => ease_in(t / 0.5),
        0.5..=1.0 => ease_out(flip(t) / 0.5),
        _ => panic!("'t' must be between 0.0 and 1.0, but it was {}", t),
    }
}

pub fn square(t: f32) -> f32 {
    t * t
}

pub fn ease_in(t: f32) -> f32 {
    square(t)
}

pub fn ease_out(t: f32) -> f32 {
    flip(square(flip(t)))
}
