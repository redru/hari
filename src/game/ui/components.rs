use bevy::prelude::*;

use crate::game::components::{CurrentLifes, CurrentScore, Lifes, Score};

pub fn top_bar() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Px(20.)),
            width: Val::Vw(100.0),
            height: Val::Px(60.0),
            column_gap: Val::Px(0.0),
            ..default()
        },
        ..default()
    }
}

pub fn group_section() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Px(20.)),
            height: Val::Percent(100.0),
            width: Val::Px(100.0),
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },
        ..default()
    }
}

pub fn gull_gizmo(asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        NodeBundle {
            style: Style {
                width: Val::Px(32.),
                height: Val::Px(25.),
                ..default()
            },
            ..default()
        },
        UiImage::new(asset_server.load("1920x1080/gull_gizmo_32x25.png")),
    )
}

pub fn score_text(
    asset_server: &Res<AssetServer>,
    current_score: &Res<CurrentScore>,
) -> impl Bundle {
    (
        TextBundle::from_section(
            format!("{}", current_score.0),
            TextStyle {
                font: asset_server.load("1920x1080/Inconsolata-VariableFont_wdth,wght.ttf"),
                font_size: 40.0,
                color: Color::BLACK,
            },
        ),
        Score,
    )
}

pub fn boat_gizmo(asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        NodeBundle {
            style: Style {
                width: Val::Px(29.),
                height: Val::Px(32.),
                ..default()
            },
            ..default()
        },
        UiImage::new(asset_server.load("1920x1080/boat_gizmo_29x32.png")),
    )
}

pub fn lifes_text(
    asset_server: &Res<AssetServer>,
    current_lifes: &Res<CurrentLifes>,
) -> impl Bundle {
    (
        TextBundle::from_section(
            format!("{}", current_lifes.0),
            TextStyle {
                font: asset_server.load("1920x1080/Inconsolata-VariableFont_wdth,wght.ttf"),
                font_size: 40.0,
                color: Color::BLACK,
            },
        ),
        Lifes,
    )
}
