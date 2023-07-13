use bevy::{prelude::*, window::WindowResolution};
use consts::{PLAYER_MAX_HEALTH, WINDOW_HEIGHT, WINDOW_WIDTH};

mod camera;
mod common;
mod consts;
mod enemy;
mod movement;
mod player;
mod ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    MainMenu,
    LoadingGame,
    #[default]
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameplayState {
    #[default]
    Playing,
    Paused,
}

pub fn is_playing(game: Res<State<GameState>>, gameplay: Res<State<GameplayState>>) -> bool {
    matches!(game.get(), GameState::InGame) && matches!(gameplay.get(), GameplayState::Playing)
}

fn main() {
    App::new()
        // --- Initial resources ---
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<SpaceshipState>()
        .init_resource::<WinSize>()
        // --- Initial game states ---
        .add_state::<GameState>()
        .add_state::<GameplayState>()
        // --- Install plugins ---
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        title: "Space Ranger Cinnamon".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
            camera::CameraPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            movement::MovementPlugin,
            common::CommonPlugin,
            ui::UiPlugin,
        ))
        .run();
}

#[derive(Resource, Debug)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

impl Default for WinSize {
    fn default() -> Self {
        Self {
            w: WINDOW_WIDTH,
            h: WINDOW_HEIGHT,
        }
    }
}

#[derive(Resource, Debug)]
pub struct SpaceshipState {
    pub health: u32,
}

impl Default for SpaceshipState {
    fn default() -> Self {
        Self {
            health: PLAYER_MAX_HEALTH,
        }
    }
}
