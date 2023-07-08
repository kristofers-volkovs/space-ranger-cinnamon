// --- Gameplay screen ---

pub const RATIO: f32 = 9.0 / 10.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT * RATIO;

pub const DESPAWN_MARGIN: f32 = 200.0;

// --- Player ---

pub const PLAYER_POSITION: f32 = -(WINDOW_HEIGHT / 2.0) * (4.0 / 5.0);

pub const PLAYER_MOVEMENT_SPEED: f32 = 5.;
pub const PLAYER_PROJECTILE_SPEED: f32 = 1000.;
pub const PLAYER_DASH_SPEED: f32 = 300.0;

pub const PLAYER_DASH_TIME_LEN: f32 = 0.5;

pub const PLAYER_FIRING_COOLDOWN: f32 = 0.2;
pub const PLAYER_DASH_COOLDOWN: f32 = 0.5;

pub const PLAYER_Z: f32 = 10.0;
pub const PLAYER_PROJECTILE_Z: f32 = 0.0;
