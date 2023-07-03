use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

#[derive(Component)]
struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((GameCamera, Camera2dBundle::default()));
}
