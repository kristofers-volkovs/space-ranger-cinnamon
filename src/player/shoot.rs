use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    common::{EntityType, ProjectileBundle, ProjectileSource},
    consts,
    movement::Velocity,
};

use super::{PlayerAssets, Spaceship, SpaceshipAction};

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
    pub fn new() -> Self {
        Self {
            state: ShootingState::Idle,
        }
    }
}

// ===

pub fn spaceship_shoot(
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
    player_assets: Res<PlayerAssets>,
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
            ShootingState::Idle | ShootingState::Charging => (),
            ShootingState::Shooting => {
                let projectile_bundle = ProjectileBundle::new(
                    EntityType::Projectile,
                    Velocity {
                        x: 0.,
                        y: consts::PLAYER_PROJECTILE_SPEED,
                    },
                    tf.translation * Vec2::ONE.extend(consts::PLAYER_PROJECTILE_Z),
                    player_assets.projectile.clone(),
                    ProjectileSource::FromSpaceship,
                );
                commands.spawn(projectile_bundle);

                spaceship_shoot.state = ShootingState::Cooldown(Timer::from_seconds(
                    consts::PLAYER_FIRING_COOLDOWN,
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
