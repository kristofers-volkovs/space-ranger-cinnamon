use bevy::prelude::*;

use crate::{consts::PLAYER_MAX_HEALTH, is_playing, GameState, SpaceshipState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_gameplay_ui)
            .add_systems(Update, spaceship_health_update.run_if(is_playing));
    }
}

// ===

#[derive(Component)]
struct GameplayUi;

#[derive(Component)]
struct HealthPoint;

impl HealthPoint {
    fn full_health_point_color() -> Color {
        Color::rgb(0.1, 1.0, 0.1)
    }

    fn empty_health_point_color() -> Color {
        Color::rgb(0.5, 0.5, 0.5)
    }
}

// ===

fn setup_gameplay_ui(mut commands: Commands, spaceship_state: Res<SpaceshipState>) {
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
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for idx in 1..=PLAYER_MAX_HEALTH {
                        parent.spawn((
                            HealthPoint,
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: {
                                    if spaceship_state.health >= idx {
                                        BackgroundColor::from(HealthPoint::full_health_point_color())
                                    } else {
                                        BackgroundColor::from(HealthPoint::empty_health_point_color())
                                    }},
                                ..default()
                            },
                        ));
                    }
                });
        });
}

fn spaceship_health_update(
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
