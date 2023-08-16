use crate::{
    consts,
    movement::{Direction, Velocity},
    WinSize,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use super::{Point, Spaceship, SpaceshipAction};

#[derive(Debug)]
pub struct Dash {
    direction: Direction,
    timer: Timer,
}

impl Dash {
    fn new(direction: Direction) -> Self {
        Self {
            direction,
            timer: Timer::from_seconds(consts::PLAYER_DASH_TIME_LEN, TimerMode::Once),
        }
    }

    fn calc_boost(&mut self, time: &Res<Time>) -> Option<f32> {
        self.timer.tick(time.delta());

        if !self.timer.finished() {
            let elapsed_secs: f32 = self.timer.elapsed_secs();

            let boost = {
                let speed = consts::PLAYER_DASH_SPEED;
                let dash_time = consts::PLAYER_DASH_TIME_LEN;
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
    pub fn new() -> Self {
        Self {
            state: DashState::Idle,
        }
    }
}

// ===

pub fn spaceship_movement(
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
                    velocity.x += consts::PLAYER_MOVEMENT_SPEED;
                }

                if action_state.pressed(SpaceshipAction::MoveLeft) {
                    velocity.x -= consts::PLAYER_MOVEMENT_SPEED;
                }
            }
            DashState::Dashing(ref mut dash) => match dash.calc_boost(&time) {
                Some(velocity_boost) => {
                    velocity.x += velocity_boost;
                }
                None => {
                    spaceship_dash.state = DashState::Cooldown(Timer::from_seconds(
                        consts::PLAYER_DASH_COOLDOWN,
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

pub fn apply_spaceship_velocity(
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
