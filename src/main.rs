use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

mod camera;
mod consts;
mod movement;
mod player;

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

fn main() {
    App::new()
        // --- Initial resources ---
        .insert_resource(ClearColor(Color::BLACK))
        // --- Initial game states ---
        .add_state::<GameState>()
        .add_state::<GameplayState>()
        // --- Initialize games resources ---
        .add_startup_system(resource_setup)
        // --- Install plugins ---
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
        .add_plugin(movement::MovementPlugin)
        .run();
}

#[derive(Resource, Debug)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

fn resource_setup(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let window = query.get_single().unwrap();
    let win_size = WinSize {
        w: window.width(),
        h: window.height(),
    };

    commands.insert_resource(win_size);
}
