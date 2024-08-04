use bevy::prelude::*;
use components::*;
use systems::*;

pub mod components;
pub mod systems;

#[derive(Bundle, Clone)]
pub struct PhysicsMovementBundle {
    pub physical_translation: PhysicalTranslation,
    pub velocity: Velocity,
    previous_physical_translation: PreviousPhysicalTranslation,
}

impl Default for PhysicsMovementBundle {
    fn default() -> Self {
        Self {
            physical_translation: PhysicalTranslation(Vec3::ZERO),
            velocity: Velocity(Vec3::ZERO),
            previous_physical_translation: PreviousPhysicalTranslation(Vec3::ZERO),
        }
    }
}

impl PhysicsMovementBundle {
    pub fn new(physical_translation: Vec3, velocity: Vec3) -> Self {
        Self {
            physical_translation: PhysicalTranslation(physical_translation.clone()),
            velocity: Velocity(velocity),
            previous_physical_translation: PreviousPhysicalTranslation(physical_translation),
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (advance_physics).in_set(PhysicsSet))
            .add_systems(Update, (update_rendered_transform).in_set(PhysicsSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;
