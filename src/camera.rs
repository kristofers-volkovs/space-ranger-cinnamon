use bevy::{prelude::*, render::camera::ScalingMode};

use crate::consts;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        GameCamera,
        Camera2dBundle {
            projection: OrthographicProjection {
                // TODO viewport scaling is weird when resizing the window width
                scaling_mode: ScalingMode::FixedVertical(consts::WINDOW_HEIGHT),
                ..default()
            },
            ..default()
        },
    ));
}
