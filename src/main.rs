use bevy::{prelude::*, window::WindowResolution};

mod camera;
mod player;

const RESOLUTION: f32 = 9.0 / 10.0;
const WINDOW_HEIGHT: f32 = 1000.0;
const WINDOW_WIDTH: f32 = WINDOW_HEIGHT * RESOLUTION;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum EngineState {
    MainMenu,
    LoadingGame,
    #[default]
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum InGameState {
    #[default]
    Playing,
    Paused,
}

fn main() {
    App::new()
        // Initial resources
        .insert_resource(ClearColor(Color::BLACK))
        // Initial game states
        .add_state::<EngineState>()
        .add_state::<InGameState>()
        // Install plugins
        .add_plugins(
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
        )
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}
