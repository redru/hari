use bevy::prelude::*;

pub fn vec2_faces_point(direction: Vec2, position: Vec2, destination: Vec2) -> bool {
    let direction_to_dest = (destination - position).normalize_or_zero();

    if direction_to_dest == Vec2::ZERO {
        return true;
    }

    direction.dot(direction_to_dest) > 0.9
}

pub fn atan2(normalized: Vec3) -> f32 {
    normalized.y.atan2(normalized.x)
}

pub fn lerp_f32(start_value: f32, end_value: f32, t: f32) -> f32 {
    debug_assert!(
        t >= 0.0 && t <= 1.0,
        "'t' must be between 0.0 and 1.0, but it was {}",
        t
    );
    start_value + (end_value - start_value) * t
}

// This must be fixed, it is wrong function
pub fn lerp_ease_in_out_quad(start_value: f32, end_value: f32, t: f32) -> f32 {
    debug_assert!(
        t >= 0.0 && t <= 1.0,
        "'t' must be between 0.0 and 1.0, but it was {}",
        t
    );
    lerp_f32(start_value, end_value, spike(ease_in_out_quad(t)))
}

fn ease_in_out_quad(t: f32) -> f32 {
    match t {
        0.0..0.5 => 2.0 * square(t),
        0.5..=1.0 => flip(square((-2.0 * t) + 2.0) / 2.0),
        _ => panic!("'t' must be between 0.0 and 1.0, but it was {}", t),
    }
}

fn flip(t: f32) -> f32 {
    debug_assert!(
        t >= 0.0 && t <= 1.0,
        "'t' must be between 0.0 and 1.0, but it was {}",
        t
    );
    1.0 - t
}

fn spike(t: f32) -> f32 {
    match t {
        0.0..0.5 => t / 0.5,
        0.5..=1.0 => flip(t) / 0.5,
        _ => panic!("'t' must be between 0.0 and 1.0, but it was {}", t),
    }
}

fn square(t: f32) -> f32 {
    debug_assert!(
        t >= 0.0 && t <= 1.0,
        "'t' must be between 0.0 and 1.0, but it was {}",
        t
    );
    t * t
}
