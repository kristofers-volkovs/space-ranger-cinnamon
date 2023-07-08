// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::consts::{
    DESPAWN_MARGIN, PLAYER_DASH_SPEED, PLAYER_DASH_TIME_LEN, PLAYER_MOVEMENT_SPEED,
    PLAYER_POSITION, PLAYER_PROJECTILE_SPEED, PLAYER_PROJECTILE_Z, PLAYER_Z,
};
use crate::movement::{Direction, Movable, MovementSet, Velocity};
use crate::{GameState, GameplayState, WinSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<SpaceshipAction>::default())
            .add_system(spawn_spaceship.in_schedule(OnEnter(GameState::InGame)))
            .add_system(
                spaceship_dash
                    .run_if(is_playing)
                    .in_set(MovementSet::InitAction),
            )
            .add_systems(
                (spaceship_movement, apply_spaceship_dash_to_velocity)
                    .distributive_run_if(is_playing)
                    .in_set(MovementSet::UpdateVelocity)
                    .after(MovementSet::InitAction),
            )
            .add_system(
                apply_spaceship_velocity
                    .run_if(is_playing)
                    .in_set(MovementSet::ApplyVelocity)
                    .after(MovementSet::UpdateVelocity),
            )
            .add_systems((spaceship_shoot, out_of_bounds_despawn).distributive_run_if(is_playing));
    }
}

pub fn is_playing(engine: Res<State<GameState>>, game: Res<State<GameplayState>>) -> bool {
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

struct Point {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
pub struct SpaceshipDash {
    direction: Direction,
    timer: Timer,
}

impl SpaceshipDash {
    fn new(direction: Direction) -> Self {
        SpaceshipDash {
            direction,
            timer: Timer::from_seconds(PLAYER_DASH_TIME_LEN, TimerMode::Once),
        }
    }

    fn calc_boost(&mut self, time: &Res<Time>) -> Option<f32> {
        self.timer.tick(time.delta());

        if !self.timer.finished() {
            let elapsed_secs: f32 = self.timer.elapsed_secs();

            let boost = {
                let speed = PLAYER_DASH_SPEED;
                let dash_time = PLAYER_DASH_TIME_LEN;
                let half_dash_time = dash_time / 2.0;

                let parabola_max =
                    -speed * half_dash_time.powi(2) + speed * dash_time * half_dash_time;

                if elapsed_secs < half_dash_time {
                    // parabola: -A*x^2 + B*x
                    // A & B - constants
                    // x - time
                    -speed * elapsed_secs.powi(2) + speed * dash_time * elapsed_secs
                } else {
                    let p1 = Point {
                        x: half_dash_time,
                        y: parabola_max,
                    };
                    let p2 = Point {
                        x: dash_time,
                        y: -parabola_max / 2.0,
                    };

                    let slope = (p2.y - p1.y) / (p2.x - p1.x);
                    let interception = p1.y - slope * p1.x;

                    // linear function: C*x + D
                    // C & D - constants
                    // x - time
                    slope * elapsed_secs + interception
                }
            };

            match self.direction {
                Direction::Right => Some(boost),
                Direction::Left => Some(-boost),
            }
        } else {
            None
        }
    }
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
            transform: Transform::from_xyz(0., PLAYER_POSITION, PLAYER_Z),
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

fn spaceship_dash(
    mut commands: Commands,
    player_query: Query<(Entity, &ActionState<SpaceshipAction>), With<Spaceship>>,
) {

    let (entity, action_state) = player_query.single();

    if action_state.just_pressed(SpaceshipAction::Dash) {
        let direction = {
            if action_state.pressed(SpaceshipAction::MoveRight) {
                Some(Direction::Right)
            } else if action_state.pressed(SpaceshipAction::MoveLeft) {
                Some(Direction::Left)
            } else {
                None
            }
        };

        if let Some(d) = direction {
            // TODO spawn dash cooldown timer
            commands.entity(entity).insert(SpaceshipDash::new(d));
        }
    }
}

fn apply_spaceship_dash_to_velocity(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Velocity, &mut SpaceshipDash), With<Spaceship>>,
    time: Res<Time>,
) {
    for (entity, mut velocity, mut spaceship_dash) in player_query.iter_mut() {
        match spaceship_dash.calc_boost(&time) {
            Some(velocity_boost) => {
                velocity.x += velocity_boost;
            }
            None => {
                commands.entity(entity).remove::<SpaceshipDash>();
            }
        }
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
        // TODO spawn shoot cooldown timer

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
