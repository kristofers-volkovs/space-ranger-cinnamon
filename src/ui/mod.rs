use bevy::prelude::*;

use crate::{common::despawn_entities, is_playing, GameState, GameplayState};

use self::{
    gameplay::GameplayPauseBtn,
    pause::{MenuExitBtn, MenuPause},
};

mod gameplay;
mod pause;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (
                gameplay::setup_gameplay_ui,
                gameplay::unpause_gameplay_watch,
            ),
        )
        .add_systems(
            Update,
            (
                gameplay::spaceship_health_update,
                gameplay::update_gameplay_watch,
                gameplay::update_gameplay_score,
                pause_gameplay.run_if(clicked_btn::<GameplayPauseBtn>.or_else(pressed_esc)),
            )
                .run_if(is_playing),
        )
        .add_systems(OnEnter(GameplayState::Paused), pause::setup_pause_menu)
        .add_systems(
            Update,
            unpause_gameplay.run_if(clicked_btn::<MenuExitBtn>.or_else(pressed_esc)),
        )
        .add_systems(OnExit(GameplayState::Paused), despawn_entities::<MenuPause>);
    }
}

fn pause_gameplay(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameplayState::Paused)));
}

fn unpause_gameplay(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameplayState::Playing)));
}

pub fn clicked_btn<T: Component>(query: Query<&Interaction, With<T>>) -> bool {
    for interaction in query.iter() {
        if let Interaction::Pressed = interaction {
            return true;
        }
    }
    false
}

pub fn pressed_esc(kdb: Res<Input<KeyCode>>) -> bool {
    kdb.just_pressed(KeyCode::Escape)
}
