use bevy::{prelude::*, window::PresentMode};
use game::GamePlugin;

mod game;

fn main() {
    let present_mode = get_present_mode();

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Hari Game".to_string(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    present_mode,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .add_systems(Update, exit_system)
        .run();
}

fn get_present_mode() -> PresentMode {
    if cfg!(unix) {
        PresentMode::AutoNoVsync
    } else {
        PresentMode::AutoVsync
    }
}

fn exit_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
