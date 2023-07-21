use bevy::prelude::*;

#[derive(Component)]
pub struct MenuPause;

#[derive(Component)]
pub struct MenuExitBtn;

// ===

pub fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuPause,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                MenuExitBtn,
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
}
