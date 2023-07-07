// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::consts::{
    DESPAWN_MARGIN, PLAYER_MOVEMENT_SPEED, PLAYER_PROJECTILE_SPEED, PLAYER_PROJECTILE_Z, PLAYER_Z,
};
use crate::{GameState, GameplayState, WinSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<SpaceshipAction>::default())
            .add_system(spawn_spaceship.in_schedule(OnEnter(GameState::InGame)))
            .add_systems(
                (
                    spaceship_movement,
                    apply_spaceship_velocity,
                    spaceship_shoot,
                    apply_velocity,
                    out_of_bounds_despawn,
                )
                    .distributive_run_if(is_playing),
            );
    }
}

fn is_playing(engine: Res<State<GameState>>, game: Res<State<GameplayState>>) -> bool {
    engine.0 == GameState::InGame && game.0 == GameplayState::Playing
}

// ===

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum SpaceshipAction {
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

#[derive(Component, Debug)]
struct Movable {
    auto_despawn: bool,
}

#[derive(Bundle)]
struct SpaceshipBundle {
    spaceship: Spaceship,
    velocity: Velocity,
    #[bundle]
    input_manager: InputManagerBundle<SpaceshipAction>,
    #[bundle]
    sprite: SpriteBundle,
}

impl SpaceshipBundle {
    fn default_input_map() -> InputMap<SpaceshipAction> {
        InputMap::new([
            (KeyCode::A, SpaceshipAction::MoveLeft),
            (KeyCode::D, SpaceshipAction::MoveRight),
            (KeyCode::LShift, SpaceshipAction::Dash),
            (KeyCode::Space, SpaceshipAction::Shoot),
            (KeyCode::Space, SpaceshipAction::ChargeShot),
        ])
    }
}

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Bundle)]
struct ProjectileBundle {
    projectile: Projectile,
    velocity: Velocity,
    movable: Movable,
    #[bundle]
    sprite: SpriteBundle,
}

// ===

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn(SpaceshipBundle {
        spaceship: Spaceship,
        velocity: Velocity::new(),
        input_manager: InputManagerBundle {
            input_map: SpaceshipBundle::default_input_map(),
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
    mut player_query: Query<(&ActionState<SpaceshipAction>, &mut Velocity), With<Spaceship>>,
) {
    let (action_state, mut velocity) = player_query.single_mut();

    if action_state.pressed(SpaceshipAction::MoveRight) {
        velocity.x += PLAYER_MOVEMENT_SPEED;
    }

    if action_state.pressed(SpaceshipAction::MoveLeft) {
        velocity.x -= PLAYER_MOVEMENT_SPEED;
    }
}

fn apply_spaceship_velocity(
    mut player_query: Query<(&mut Transform, &Velocity), With<Spaceship>>,
    time: Res<Time>,
    win_size: Res<WinSize>,
) {
    let (mut transform, velocity) = player_query.single_mut();

    let w_bound = win_size.w / 2.;
    if (-w_bound > transform.translation.x && velocity.x < 0.)
        || (w_bound < transform.translation.x && velocity.x > 0.)
    {
        // TODO ships velocity should get back to 0 faster when on the border
        return;
    }

    transform.translation.x += velocity.x * time.delta_seconds();
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), Without<Spaceship>>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn out_of_bounds_despawn(
    mut commands: Commands,
    win_size: Res<WinSize>,
    query: Query<(Entity, &Transform, &Movable)>,
) {
    for (entity, transform, movable) in query.iter() {
        if movable.auto_despawn {
            let translation = transform.translation;
            let h_bound = win_size.h / 2. + DESPAWN_MARGIN;
            let w_bound = win_size.w / 2. + DESPAWN_MARGIN;

            if translation.y > h_bound
                || translation.y < -h_bound
                || translation.x > w_bound
                || translation.x < -w_bound
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn spaceship_shoot(
    mut commands: Commands,
    player_query: Query<(&ActionState<SpaceshipAction>, &Transform), With<Spaceship>>,
) {
    let (action_state, transform) = player_query.single();

    if action_state.just_pressed(SpaceshipAction::Shoot) {
        commands.spawn(ProjectileBundle {
            projectile: Projectile,
            velocity: Velocity {
                x: 0.,
                y: PLAYER_PROJECTILE_SPEED,
            },
            movable: Movable { auto_despawn: true },
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
