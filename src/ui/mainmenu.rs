use bevy::prelude::*;

use crate::FontHandle;

#[derive(Component)]
pub struct MainMenuUi;

#[derive(Component)]
pub struct MainMenuPlayBtn;

#[derive(Component)]
pub struct MainMenuExitBtn;

// ===

pub fn setup_main_menu_ui(mut commands: Commands, font: Res<FontHandle>) {
    commands
        .spawn((
            MainMenuUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    MainMenuPlayBtn,
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
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
                    MainMenuExitBtn,
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}
