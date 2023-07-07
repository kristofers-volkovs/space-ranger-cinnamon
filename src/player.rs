// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::consts::PLAYER_MOVEMENT_SPEED;
use crate::{EngineState, InGameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(spawn_player.in_schedule(OnEnter(EngineState::InGame)))
            .add_systems((spaceship_movement, apply_velocity).distributive_run_if(is_playing));
    }
}

fn is_playing(engine: Res<State<EngineState>>, game: Res<State<InGameState>>) -> bool {
    engine.0 == EngineState::InGame && game.0 == InGameState::Playing
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

// ===

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player,
        velocity: Velocity { x: 0.0 },
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
            transform: Transform::from_xyz(0., 0., 0.),
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
    }
}
