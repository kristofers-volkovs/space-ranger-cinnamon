#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// --- Gameplay screen ---

pub const RATIO: f32 = 9.0 / 10.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT * RATIO;

pub const DESPAWN_MARGIN: f32 = 200.0;

// --- Player ---

pub const PLAYER_MOVEMENT_SPEED: f32 = 10.;
pub const PLAYER_PROJECTILE_SPEED: f32 = 500.;

pub const PLAYER_Z: f32 = 10.0;
pub const PLAYER_PROJECTILE_Z: f32 = 0.0;
