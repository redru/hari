use bevy::prelude::*;

/// The value represents the ratio. The value of 1.0 means 9.8 m/s2
#[derive(Debug, Component, Clone, Copy, PartialEq, Deref, DerefMut)]
pub struct Gravity(pub f32);

impl Gravity {
    pub fn new(ratio: f32) -> Self {
        Self(ratio)
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self(1.0)
    }
}

/// How many units per second the player should move.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `update_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

#[derive(Component)]
pub struct RectangleCollider {
    pub enabled: bool,
    pub width: f32,
    pub height: f32,
}

impl RectangleCollider {
    pub fn new(enabled: bool, width: f32, height: f32) -> Self {
        Self {
            enabled,
            width,
            height,
        }
    }
}
