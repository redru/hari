use bevy::prelude::*;

pub fn camera_startup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 60., 0.),
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 1.25,
            ..default()
        },
        ..default()
    });
}
