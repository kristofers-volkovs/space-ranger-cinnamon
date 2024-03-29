// --- Gameplay screen ---

pub const WINDOW_HEIGHT: f32 = 240.0 * 3.0;
pub const WINDOW_WIDTH: f32 = 256.0 * 3.0;
pub const WINDOW_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

pub const SPAWN_MARGIN: f32 = 100.0;
pub const DESPAWN_MARGIN: f32 = 200.0;

// --- Player ---

pub const PLAYER_MAX_HEALTH: u32 = 3;

pub const PLAYER_MOVEMENT_SPEED: f32 = 5.;
pub const PLAYER_PROJECTILE_SPEED: f32 = 1000.;
pub const PLAYER_DASH_SPEED: f32 = 300.0;

pub const PLAYER_DASH_TIME_LEN: f32 = 0.5;

pub const PLAYER_CHARGE_SHOT_CHARGING_TIME: f32 = 0.2;

pub const PLAYER_FIRING_COOLDOWN: f32 = 0.1;
pub const PLAYER_CHARGE_SHOT_COOLDOWN: f32 = 1.0;
pub const PLAYER_DASH_COOLDOWN: f32 = 0.3;

pub const PLAYER_CHARGE_SHOT_WIDTH: f32 = 6.0;
pub const PLAYER_CHARGE_SHOT_HEIGHT: f32 = WINDOW_HEIGHT;

pub const PLAYER_INVULNERABILITY_TIME: f32 = 3.0;
pub const PLAYER_INVULNERABILITY_ANIMATION_TIME: f32 = 0.2;

pub const PLAYER_Z: f32 = 10.0;
pub const PLAYER_PROPULSION_Z: f32 = 11.0;
pub const PLAYER_PROJECTILE_Z: f32 = 1.0;
pub const PLAYER_CHARGE_SHOT_Z: f32 = 2.0;

pub const PLAYER_SPRITE_SPACESHIP: &str = "sprites/spaceship.png";
pub const PLAYER_SPRITE_PROJECTILE: &str = "sprites/spaceship-projectile.png";
pub const PLAYER_ASEPRITE_PROPULSION: &str = "aseprites/spaceship-propulsion.aseprite";

// --- Enemy ---

pub const ENEMY_Z: f32 = 0.0;

// --- Score ---

pub const SCORE_ADD_ASTEROID: u32 = 1;

// --- Stage ---

pub const STAGE_INIT_COOLDOWN: f32 = 3.0;
pub const STAGE_COOLDOWN: f32 = 5.0;
pub const STAGE_LENGTH: f32 = 30.0;
