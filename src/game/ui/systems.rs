use bevy::prelude::*;

use crate::game::components::{CurrentLifes, CurrentScore, Score, SeagullCaught};

use super::components::{boat_gizmo, group_section, gull_gizmo, lifes_text, score_text, top_bar};

pub fn ui_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_score: Res<CurrentScore>,
    current_lifes: Res<CurrentLifes>,
) {
    commands.spawn(top_bar()).with_children(|parent| {
        parent.spawn(group_section()).with_children(|parent| {
            parent.spawn(gull_gizmo(&asset_server));
            parent
                .spawn(NodeBundle {
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(score_text(&asset_server, &current_score));
                });
        });

        parent.spawn(group_section()).with_children(|parent| {
            parent.spawn(boat_gizmo(&asset_server));
            parent
                .spawn(NodeBundle {
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(lifes_text(&asset_server, &current_lifes));
                });
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
