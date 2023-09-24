use bevy::prelude::*;
use bevy::reflect::TypePath;
use leafwing_input_manager::prelude::*;

use crate::common::EntityType;
use crate::consts;
use crate::events::EventSet;
use crate::movement::{MovementSet, Velocity};
use crate::{is_playing, GameState, WinSize};

mod movement;
mod shoot;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<SpaceshipAction>::default())
            .add_systems(
                OnEnter(GameState::Gameplay),
                (load_player_asset_dimensions, spawn_spaceship),
            )
            .add_systems(
                Update,
                (
                    (
                        shoot::spaceship_shoot,
                        shoot::charged_shot_hit_detection,
                        spaceship_invincibility,
                    ),
                    movement::spaceship_movement
                        .in_set(MovementSet::UpdateVelocity)
                        .after(EventSet::HandleDespawn),
                    movement::apply_spaceship_velocity
                        .in_set(MovementSet::ApplyVelocity)
                        .after(MovementSet::UpdateVelocity),
                )
                    .run_if(is_playing),
            );
    }
}

// ===

#[derive(Component, Debug)]
pub struct SpaceshipHealth(pub u32);

#[derive(Component, Debug)]
pub struct Spaceship;

impl Spaceship {
    pub fn player_position(window_height: f32) -> f32 {
        -(window_height / 2.0) * (4.0 / 5.0)
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum SpaceshipAction {
    MoveRight,
    MoveLeft,
    DashRight,
    DashLeft,
    Shoot,
}

impl SpaceshipAction {
    fn default_input_map() -> InputMap<SpaceshipAction> {
        InputMap::new([
            (KeyCode::A, SpaceshipAction::MoveLeft),
            (KeyCode::D, SpaceshipAction::MoveRight),
            (KeyCode::E, SpaceshipAction::DashRight),
            (KeyCode::Q, SpaceshipAction::DashLeft),
            (KeyCode::Space, SpaceshipAction::Shoot),
        ])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[component(storage = "SparseSet")]
pub struct Invulnerability {
    pub length: f32,
    pub animation_timer: Timer,
}

impl Invulnerability {
    pub fn new() -> Self {
        Self {
            length: consts::PLAYER_INVULNERABILITY_TIME,
            animation_timer: Timer::from_seconds(
                consts::PLAYER_INVULNERABILITY_ANIMATION_TIME,
                TimerMode::Repeating,
            ),
        }
    }
}

#[derive(Bundle)]
struct SpaceshipBundle {
    spaceship: Spaceship,
    entity_type: EntityType,
    health: SpaceshipHealth,
    velocity: Velocity,
    dash: movement::SpaceshipDash,
    shooting: shoot::SpaceshipShoot,
    #[bundle()]
    input_manager: InputManagerBundle<SpaceshipAction>,
    #[bundle()]
    sprite: SpriteBundle,
}

#[derive(Resource)]
pub struct PlayerHandles {
    pub spaceship: Handle<Image>,
    pub projectile: Handle<Image>,
}

#[derive(Resource)]
pub struct PlayerAssetDimensions {
    pub spaceship: Vec2,
    pub projectile: Vec2,
}

// ===

fn spawn_spaceship(
    mut commands: Commands,
    player_assets: Res<PlayerHandles>,
    win_size: Res<WinSize>,
) {
    commands.spawn(SpaceshipBundle {
        spaceship: Spaceship,
        entity_type: EntityType::Spaceship,
        health: SpaceshipHealth(consts::PLAYER_MAX_HEALTH),
        velocity: Velocity::new(0.0, 0.0),
        dash: movement::SpaceshipDash::new(),
        shooting: shoot::SpaceshipShoot::new(),
        input_manager: InputManagerBundle {
            input_map: SpaceshipAction::default_input_map(),
            ..default()
        },
        sprite: SpriteBundle {
            texture: player_assets.spaceship.clone(),
            transform: Transform::from_xyz(
                0.,
                Spaceship::player_position(win_size.h),
                consts::PLAYER_Z,
            ),
            ..default()
        },
    });
}

fn spaceship_invincibility(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Invulnerability, &mut Sprite), With<Spaceship>>,
) {
    if let Ok((entity, mut invincibility, mut sprite)) = query.get_single_mut() {
        invincibility.length -= time.delta_seconds();
        invincibility.animation_timer.tick(time.delta());

        if invincibility.animation_timer.finished() {
            match sprite.color.a() {
                a if a == 1.0 => sprite.color.set_a(0.3),
                _ => sprite.color.set_a(1.0),
            };
        }

        if invincibility.length <= 0.0 {
            commands.entity(entity).remove::<Invulnerability>();
            sprite.color.set_a(1.0);
        }
    }
}

pub fn load_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = PlayerHandles {
        spaceship: asset_server.load("sprites/spaceship.png"),
        projectile: asset_server.load("sprites/spaceship-projectile.png"),
    };

    commands.insert_resource(assets);
}

pub fn load_player_asset_dimensions(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    player_assets: Res<PlayerHandles>,
) {
    let spaceship_size = match images.get(&player_assets.spaceship) {
        Some(image) => image.size(),
        None => return,
    };
    let projectile_size = match images.get(&player_assets.projectile) {
        Some(image) => image.size(),
        None => return,
    };

    commands.insert_resource(PlayerAssetDimensions {
        spaceship: spaceship_size,
        projectile: projectile_size,
    });
}
