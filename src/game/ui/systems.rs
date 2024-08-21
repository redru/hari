use bevy::prelude::*;

use crate::game::components::{CurrentScore, Score, SeagullCaught};

pub fn ui_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_score: ResMut<CurrentScore>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(20.)),
                width: Val::Vw(100.0),
                height: Val::Px(60.0),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(32.),
                        height: Val::Px(25.),
                        ..default()
                    },
                    ..default()
                },
                UiImage::new(asset_server.load("1920x1080/gull_gizmo_32x25.png")),
            ));

            parent
                .spawn(NodeBundle {
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("{}", current_score.0),
                            TextStyle {
                                font: asset_server
                                    .load("1920x1080/Inconsolata-VariableFont_wdth,wght.ttf"),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        ),
                        Score,
                    ));
                });
        });
}

pub fn update_score_system(
    mut er_seagull_caught: EventReader<SeagullCaught>,
    mut current_score: ResMut<CurrentScore>,
    mut score_query: Query<&mut Text, With<Score>>,
) {
    for ev in er_seagull_caught.read() {
        current_score.0 += ev.score;
    }

    score_query.single_mut().sections.get_mut(0).unwrap().value = format!("{}", current_score.0);
}
