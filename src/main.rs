use bevy::{prelude::*, window::WindowResolution};

const RESOLUTION: f32 = 9.0 / 14.0;
const WINDOW_HEIGHT: f32 = 1000.0;
const WINDOW_WIDTH: f32 = WINDOW_HEIGHT * RESOLUTION;

fn main() {
    App::new()
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
        .add_startup_system(setup_system)
        .run();
}

#[derive(Component)]
struct GameCamera;

fn setup_system(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}
