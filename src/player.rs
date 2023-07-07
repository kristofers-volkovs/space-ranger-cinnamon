// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::consts::{
    PLAYER_MOVEMENT_SPEED, PLAYER_PROJECTILE_SPEED, PLAYER_PROJECTILE_Z, PLAYER_Z,
};
use crate::{GameState, GameplayState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(spawn_player.in_schedule(OnEnter(GameState::InGame)))
            .add_systems(
                (spaceship_movement, spaceship_shoot, apply_velocity)
                    .distributive_run_if(is_playing),
            );
    }
}

fn is_playing(engine: Res<State<GameState>>, game: Res<State<GameplayState>>) -> bool {
    engine.0 == GameState::InGame && game.0 == GameplayState::Playing
}

// ===

#[derive(Component, Debug)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveRight,
    MoveLeft,
    Dash,
    Shoot,
    ChargeShot,
}

#[derive(Component, Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    fn new() -> Velocity {
        Velocity { x: 0., y: 0. }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    velocity: Velocity,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
    #[bundle]
    sprite: SpriteBundle,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        InputMap::new([
            (KeyCode::A, PlayerAction::MoveLeft),
            (KeyCode::D, PlayerAction::MoveRight),
            (KeyCode::LShift, PlayerAction::Dash),
            (KeyCode::Space, PlayerAction::Shoot),
            (KeyCode::Space, PlayerAction::ChargeShot),
        ])
    }
}

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Bundle)]
struct ProjectileBundle {
    projectile: Projectile,
    velocity: Velocity,
    #[bundle]
    sprite: SpriteBundle,
}

// ===

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player,
        velocity: Velocity::new(),
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            ..default()
        },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2 { x: 50., y: 50. }),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., PLAYER_Z),
            ..default()
        },
    });
}

fn spaceship_movement(
    mut player_query: Query<(&ActionState<PlayerAction>, &mut Velocity), With<Player>>,
) {
    let (action_state, mut velocity) = player_query.single_mut();

    if action_state.pressed(PlayerAction::MoveRight) {
        velocity.x += PLAYER_MOVEMENT_SPEED;
    }

    if action_state.pressed(PlayerAction::MoveLeft) {
        velocity.x -= PLAYER_MOVEMENT_SPEED;
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn spaceship_shoot(
    mut commands: Commands,
    player_query: Query<(&ActionState<PlayerAction>, &Transform), With<Player>>,
) {
    let (action_state, transform) = player_query.single();

    if action_state.just_pressed(PlayerAction::Shoot) {
        commands.spawn(ProjectileBundle {
            projectile: Projectile,
            velocity: Velocity {
                x: 0.,
                y: PLAYER_PROJECTILE_SPEED,
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 0.5, 1.),
                    custom_size: Some(Vec2 { x: 10., y: 20. }),
                    ..default()
                },
                transform: Transform::from_translation(
                    transform.translation * Vec2::ONE.extend(PLAYER_PROJECTILE_Z),
                ),
                ..default()
            },
        });
    }
}
