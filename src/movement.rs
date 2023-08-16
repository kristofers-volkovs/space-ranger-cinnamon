use bevy::prelude::*;

use crate::{is_playing, player::Spaceship};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
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

impl Movable {
    pub fn new(auto_despawn: bool) -> Self {
        Movable { auto_despawn }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity { x, y }
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
    for (mut tf, velocity) in query.iter_mut() {
        tf.translation.x += velocity.x * time.delta_seconds();
        tf.translation.y += velocity.y * time.delta_seconds();
    }
}
