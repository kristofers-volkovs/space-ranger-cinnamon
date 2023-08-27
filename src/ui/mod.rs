use bevy::{app::AppExit, prelude::*};

use crate::{
    common::EntityType,
    despawn_entities, is_gameplay, is_playing,
    player::{load_player_asset_dimensions, load_player_assets, PlayerAssetDimensions},
    GameState, GameplayState,
};

mod gameplay;
mod mainmenu;
mod pause;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // === Menu ===
            .add_systems(OnEnter(GameState::MainMenu), mainmenu::setup_main_menu_ui)
            .add_systems(
                Update,
                (
                    (game_to_loading_assets, unpause_gameplay)
                        .run_if(clicked_btn::<mainmenu::MainMenuPlayBtn>),
                    exit_app.run_if(clicked_btn::<mainmenu::MainMenuExitBtn>),
                ),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_entities::<mainmenu::MainMenuUi>,
            )
            // === Loading ===
            .add_systems(OnEnter(GameState::LoadingGame), load_player_assets)
            .add_systems(
                Update,
                (
                    load_player_asset_dimensions,
                    game_to_gameplay.run_if(resource_exists::<PlayerAssetDimensions>()),
                )
                    .run_if(in_state(GameState::LoadingGame)),
            )
            // === Gameplay ===
            .add_systems(
                OnEnter(GameState::Gameplay),
                (
                    gameplay::setup_gameplay_ui,
                    gameplay::unpause_gameplay_watch,
                ),
            )
            .add_systems(
                Update,
                gameplay::spaceship_health_update.run_if(is_gameplay),
            )
            .add_systems(
                Update,
                (
                    gameplay::update_gameplay_watch,
                    gameplay::update_gameplay_score,
                    pause_gameplay
                        .run_if(clicked_btn::<gameplay::GameplayPauseBtn>.or_else(pressed_esc)),
                )
                    .run_if(is_playing),
            )
            .add_systems(
                OnExit(GameState::Gameplay),
                (
                    despawn_entities::<gameplay::GameplayUi>,
                    despawn_entities::<pause::MenuPause>,
                    despawn_entities::<EntityType>,
                    gameplay::reset_gameplay_stats,
                ),
            )
            // === Gameplay Pause ===
            .add_systems(OnEnter(GameplayState::Paused), pause::setup_pause_menu)
            .add_systems(
                Update,
                (
                    unpause_gameplay
                        .run_if(clicked_btn::<pause::MenuCloseBtn>.or_else(pressed_esc)),
                    game_to_main_menu.run_if(clicked_btn::<pause::MenuExitBtn>),
                ),
            )
            .add_systems(
                OnExit(GameplayState::Paused),
                despawn_entities::<pause::MenuPause>,
            );
    }
}

fn pause_gameplay(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameplayState::Paused)));
}

fn unpause_gameplay(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameplayState::Playing)));
}

fn game_to_main_menu(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameState::MainMenu)));
}

fn game_to_loading_assets(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameState::LoadingGame)));
}

fn game_to_gameplay(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameState::Gameplay)));
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

pub fn exit_app(mut ev_exit: EventWriter<AppExit>) {
    ev_exit.send(AppExit);
}
