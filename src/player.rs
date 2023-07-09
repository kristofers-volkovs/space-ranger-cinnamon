// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::common::{DespawnEntity, EntityType, EventSet};
use crate::consts::{
    DESPAWN_MARGIN, PLAYER_DASH_COOLDOWN, PLAYER_DASH_SPEED, PLAYER_DASH_TIME_LEN,
    PLAYER_FIRING_COOLDOWN, PLAYER_MOVEMENT_SPEED, PLAYER_POSITION, PLAYER_PROJECTILE_SPEED,
    PLAYER_PROJECTILE_Z, PLAYER_Z,
};
use crate::movement::{Direction, Movable, MovementSet, Velocity};
use crate::{is_playing, GameState, WinSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<SpaceshipAction>::default())
            .add_system(spawn_spaceship.in_schedule(OnEnter(GameState::InGame)))
            .add_system(
                spaceship_movement
                    .run_if(is_playing)
                    .in_set(MovementSet::UpdateVelocity)
                    .after(EventSet::HandleEvents),
            )
            .add_system(
                apply_spaceship_velocity
                    .run_if(is_playing)
                    .in_set(MovementSet::ApplyVelocity)
                    .after(MovementSet::UpdateVelocity),
            )
            .add_system(
                out_of_bounds_detection
                    .run_if(is_playing)
                    .in_set(EventSet::SpawnEvents),
            )
            .add_system(spaceship_shoot.run_if(is_playing));
    }
}

// ===

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum SpaceshipAction {
    MoveRight,
    MoveLeft,
    DashRight,
    DashLeft,
    Shoot,
}

impl SpaceshipBundle {
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

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Component, Debug)]
pub enum ProjectileSource {
    FromSpaceship,
    // FromEnemy,
}

#[derive(Bundle)]
struct ProjectileBundle {
    projectile: Projectile,
    entity_type: EntityType,
    velocity: Velocity,
    movable: Movable,
    source: ProjectileSource,
    #[bundle]
    sprite: SpriteBundle,
}

struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct Dash {
    direction: Direction,
    timer: Timer,
}

impl Dash {
    fn new(direction: Direction) -> Self {
        Self {
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

#[derive(Debug)]
enum DashState {
    Idle,
    Dashing(Dash),
    Cooldown(Timer),
}

impl DashState {
    fn is_idle(&self) -> bool {
        matches!(self, DashState::Idle)
    }
}

#[derive(Component, Debug)]
pub struct SpaceshipDash {
    state: DashState,
}

impl SpaceshipDash {
    fn new() -> Self {
        Self {
            state: DashState::Idle,
        }
    }
}

#[derive(Debug)]
enum ShootingState {
    Idle,
    Charging,
    Shooting,
    Cooldown(Timer),
}

impl ShootingState {
    fn is_idle(&self) -> bool {
        matches!(self, ShootingState::Idle)
    }

    fn is_charging(&self) -> bool {
        matches!(self, ShootingState::Charging)
    }
}

#[derive(Component, Debug)]
pub struct SpaceshipShoot {
    state: ShootingState,
}

impl SpaceshipShoot {
    fn new() -> Self {
        Self {
            state: ShootingState::Idle,
        }
    }
}

#[derive(Bundle)]
struct SpaceshipBundle {
    spaceship: Spaceship,
    entity_type: EntityType,
    velocity: Velocity,
    dash: SpaceshipDash,
    shooting: SpaceshipShoot,
    #[bundle]
    input_manager: InputManagerBundle<SpaceshipAction>,
    #[bundle]
    sprite: SpriteBundle,
}

// ===

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn(SpaceshipBundle {
        spaceship: Spaceship,
        entity_type: EntityType::Spaceship,
        velocity: Velocity::new(),
        dash: SpaceshipDash::new(),
        shooting: SpaceshipShoot::new(),
        input_manager: InputManagerBundle {
            input_map: SpaceshipBundle::default_input_map(),
            ..default()
        },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0., PLAYER_POSITION, PLAYER_Z),
            ..default()
        },
    });
}

fn spaceship_movement(
    mut player_query: Query<
        (
            &ActionState<SpaceshipAction>,
            &mut Velocity,
            &mut SpaceshipDash,
        ),
        With<Spaceship>,
    >,
    time: Res<Time>,
) {
    if let Ok((action_state, mut velocity, mut spaceship_dash)) = player_query.get_single_mut() {
        if spaceship_dash.state.is_idle() {
            let direction = {
                if action_state.just_pressed(SpaceshipAction::DashRight) {
                    Some(Direction::Right)
                } else if action_state.just_pressed(SpaceshipAction::DashLeft) {
                    Some(Direction::Left)
                } else {
                    None
                }
            };

            if let Some(d) = direction {
                spaceship_dash.state = DashState::Dashing(Dash::new(d));
            }
        }

        match &mut spaceship_dash.state {
            DashState::Idle => {
                if action_state.pressed(SpaceshipAction::MoveRight) {
                    velocity.x += PLAYER_MOVEMENT_SPEED;
                }

                if action_state.pressed(SpaceshipAction::MoveLeft) {
                    velocity.x -= PLAYER_MOVEMENT_SPEED;
                }
            }
            DashState::Dashing(ref mut dash) => match dash.calc_boost(&time) {
                Some(velocity_boost) => {
                    velocity.x += velocity_boost;
                }
                None => {
                    spaceship_dash.state = DashState::Cooldown(Timer::from_seconds(
                        PLAYER_DASH_COOLDOWN,
                        TimerMode::Once,
                    ));
                }
            },
            DashState::Cooldown(ref mut timer) => {
                timer.tick(time.delta());

                if timer.finished() {
                    spaceship_dash.state = DashState::Idle;
                }
            }
        }
    }
}

fn apply_spaceship_velocity(
    mut player_query: Query<(&mut Transform, &Velocity), With<Spaceship>>,
    time: Res<Time>,
    win_size: Res<WinSize>,
) {
    if let Ok((mut tf, velocity)) = player_query.get_single_mut() {
        let w_bound = win_size.w / 2.;
        if (-w_bound > tf.translation.x && velocity.x < 0.)
            || (w_bound < tf.translation.x && velocity.x > 0.)
        {
            // TODO ships velocity should get back to 0 faster when on the border
            return;
        }

        tf.translation.x += velocity.x * time.delta_seconds();
    }
}

fn out_of_bounds_detection(
    mut ev_despawn: EventWriter<DespawnEntity>,
    win_size: Res<WinSize>,
    query: Query<(Entity, &Transform, &Movable, &EntityType)>,
) {
    for (entity, tf, movable, entity_type) in query.iter() {
        if movable.auto_despawn {
            let translation = tf.translation;
            let h_bound = win_size.h / 2.0 + DESPAWN_MARGIN;
            let w_bound = win_size.w / 2.0 + DESPAWN_MARGIN;

            if translation.y > h_bound
                || translation.y < -h_bound
                || translation.x > w_bound
                || translation.x < -w_bound
            {
                ev_despawn.send(DespawnEntity {
                    entity,
                    entity_type: *entity_type,
                });
            }
        }
    }
}

fn spaceship_shoot(
    mut commands: Commands,
    mut player_query: Query<
        (
            &ActionState<SpaceshipAction>,
            &Transform,
            &mut SpaceshipShoot,
        ),
        With<Spaceship>,
    >,
    time: Res<Time>,
) {
    if let Ok((action_state, tf, mut spaceship_shoot)) = player_query.get_single_mut() {
        if spaceship_shoot.state.is_idle() && action_state.just_pressed(SpaceshipAction::Shoot) {
            spaceship_shoot.state = ShootingState::Charging;
        }

        if spaceship_shoot.state.is_charging() && action_state.just_released(SpaceshipAction::Shoot)
        {
            spaceship_shoot.state = ShootingState::Shooting;
        }

        match &mut spaceship_shoot.state {
            ShootingState::Idle => (),
            ShootingState::Charging => (),
            ShootingState::Shooting => {
                commands.spawn(ProjectileBundle {
                    projectile: Projectile,
                    entity_type: EntityType::Projectile,
                    velocity: Velocity {
                        x: 0.,
                        y: PLAYER_PROJECTILE_SPEED,
                    },
                    movable: Movable { auto_despawn: true },
                    source: ProjectileSource::FromSpaceship,
                    sprite: SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 1.0, 0.0),
                            custom_size: Some(Vec2 { x: 15.0, y: 15.0 }),
                            ..default()
                        },
                        transform: Transform::from_translation(
                            tf.translation * Vec2::ONE.extend(PLAYER_PROJECTILE_Z),
                        ),
                        ..default()
                    },
                });

                spaceship_shoot.state = ShootingState::Cooldown(Timer::from_seconds(
                    PLAYER_FIRING_COOLDOWN,
                    TimerMode::Once,
                ));
            }
            ShootingState::Cooldown(ref mut timer) => {
                timer.tick(time.delta());

                if timer.finished() {
                    spaceship_shoot.state = ShootingState::Idle;
                }
            }
        }
    }
}
