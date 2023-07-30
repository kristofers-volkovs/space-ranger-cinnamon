use bevy::prelude::*;

use crate::{consts, player::SpaceshipState, Stats};

#[derive(Component)]
pub struct GameplayUi;

#[derive(Component)]
pub struct HealthPoint;

impl HealthPoint {
    fn full_health_point_color() -> Color {
        Color::rgb(0.1, 1.0, 0.1)
    }

    fn empty_health_point_color() -> Color {
        Color::rgb(0.5, 0.5, 0.5)
    }
}

#[derive(Component)]
pub struct GameplayTime;

#[derive(Component)]
pub struct GameplayScore;

#[derive(Component)]
pub struct GameplayPauseBtn;

// ===

pub fn unpause_gameplay_watch(mut stats: ResMut<Stats>) {
    stats.watch.unpause();
}

pub fn reset_gameplay_stats(mut stats: ResMut<Stats>) {
    stats.watch.pause();
    stats.watch.reset();
    stats.score = 0;
}

pub fn setup_gameplay_ui(
    mut commands: Commands,
    spaceship_state: Res<SpaceshipState>,
    stats: Res<Stats>,
) {
    commands
        .spawn((
            GameplayUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle { ..default() })
                        .with_children(|parent| {
                            for idx in 1..=consts::PLAYER_MAX_HEALTH {
                                parent.spawn((
                                    HealthPoint,
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(30.0),
                                            height: Val::Px(30.0),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..default()
                                        },
                                        border_color: Color::BLUE.into(),
                                        background_color: {
                                            if spaceship_state.health >= idx {
                                                BackgroundColor::from(
                                                    HealthPoint::full_health_point_color(),
                                                )
                                            } else {
                                                BackgroundColor::from(
                                                    HealthPoint::empty_health_point_color(),
                                                )
                                            }
                                        },
                                        ..default()
                                    },
                                ));
                            }
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                GameplayScore,
                                TextBundle::from_section(
                                    stats.score.to_string(),
                                    TextStyle {
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                            ));
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                GameplayTime,
                                TextBundle::from_section(
                                    stats.get_watch_time(),
                                    TextStyle {
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                            ));
                        });
                })
                .with_children(|parent| {
                    parent.spawn((
                        GameplayPauseBtn,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn spaceship_health_update(
    spaceship_state: Res<SpaceshipState>,
    mut ui_query: Query<&mut BackgroundColor, With<HealthPoint>>,
) {
    for (idx, mut ui_element) in ui_query.iter_mut().enumerate() {
        if idx >= spaceship_state.health as usize {
            ui_element.0 = HealthPoint::empty_health_point_color();
        } else {
            ui_element.0 = HealthPoint::full_health_point_color();
        }
    }
}

pub fn update_gameplay_watch(
    mut stats: ResMut<Stats>,
    mut ui_query: Query<&mut Text, With<GameplayTime>>,
    time: Res<Time>,
) {
    if let Ok(mut ui_element) = ui_query.get_single_mut() {
        stats.watch.tick(time.delta());

        ui_element.sections[0].value = stats.get_watch_time();
    }
}

pub fn update_gameplay_score(
    stats: Res<Stats>,
    mut ui_query: Query<&mut Text, With<GameplayScore>>,
) {
    if let Ok(mut ui_element) = ui_query.get_single_mut() {
        ui_element.sections[0].value = stats.score.to_string();
    }
}
