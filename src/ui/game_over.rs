use crate::{FontHandle, Stats};
use bevy::prelude::*;

#[derive(Component)]
pub struct MenuGameOver;

#[derive(Component)]
pub struct MenuPlayAgainBtn;

#[derive(Component)]
pub struct MenuQuitBtn;

// ===

pub fn setup_game_over_menu(mut commands: Commands, font: Res<FontHandle>, stats: Res<Stats>) {
    commands
        .spawn((
            MenuGameOver,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: font.0.clone(),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
            ));
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("ADVENTURE LASTED: {}", stats.get_watch_time()),
                TextStyle {
                    font: font.0.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("SCORE: {}", stats.score),
                TextStyle {
                    font: font.0.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    MenuPlayAgainBtn,
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(5.0)),
                            height: Val::Px(50.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "TRY AGAIN",
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    MenuQuitBtn,
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(5.0)),
                            height: Val::Px(50.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "QUIT",
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}
