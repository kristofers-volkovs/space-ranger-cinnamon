use bevy::prelude::*;

use crate::player::{is_playing, Spaceship};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            apply_velocity
                .run_if(is_playing)
                .in_set(MovementSet::ApplyVelocity)
                .after(MovementSet::UpdateVelocity),
        );
    }
}

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub enum MovementSet {
    UpdateVelocity,
    ApplyVelocity,
}

#[derive(Component, Debug)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity { x: 0., y: 0. }
    }
}

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
}

// ===

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), Without<Spaceship>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
