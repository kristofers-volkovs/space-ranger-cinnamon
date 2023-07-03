use bevy::{prelude::*, window::WindowResolution};

mod camera;
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
        .add_plugin(camera::CameraPlugin)
        .run();
}
