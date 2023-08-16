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
    Charging(Timer),
    Shooting(EntityType),
    Cooldown(Timer),
}

impl ShootingState {
    fn is_idle(&self) -> bool {
        matches!(self, ShootingState::Idle)
    }

    fn is_charging_finished(&self) -> bool {
        if let ShootingState::Charging(timer) = self {
            timer.finished()
        } else {
            false
        }
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

#[derive(Component, Clone, Copy, Debug)]
pub struct DamageArea {
    width: f32,
    height: f32,
}

impl DamageArea {
    pub fn new() -> Self {
        Self {
            width: consts::PLAYER_CHARGE_SHOT_WIDTH,
            height: consts::PLAYER_CHARGE_SHOT_HEIGHT,
        }
    }
}

#[derive(Bundle)]
struct ChargeShotBundle {
    damage_area: DamageArea,
    #[bundle()]
    sprite: SpriteBundle,
}

impl ChargeShotBundle {
    fn new(spaceship_tf: Vec2) -> Self {
        let damage_area = DamageArea::new();
        let spawn_point = Vec3::new(
            spaceship_tf.x,
            spaceship_tf.y + damage_area.height / 2.0,
            consts::PLAYER_CHARGE_SHOT_Z,
        );

        ChargeShotBundle {
            damage_area,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.9, 0.9, 0.9),
                    custom_size: Some(Vec2::new(damage_area.width, damage_area.height)),
                    ..default()
                },
                transform: Transform::from_translation(spawn_point),
                ..default()
            },
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
            spaceship_shoot.state = ShootingState::Charging(Timer::from_seconds(
                consts::PLAYER_CHARGE_SHOT_CHARGING_TIME,
                TimerMode::Once,
            ));
        }

        if action_state.just_released(SpaceshipAction::Shoot) {
            if spaceship_shoot.state.is_charging_finished() {
                spaceship_shoot.state = ShootingState::Shooting(EntityType::ChargedShot);
            } else {
                spaceship_shoot.state = ShootingState::Shooting(EntityType::Projectile);
            }
        }

        match &mut spaceship_shoot.state {
            ShootingState::Idle => (),
            ShootingState::Charging(ref mut timer) => {
                timer.tick(time.delta());
            }
            ShootingState::Shooting(ref entity_type) => match entity_type {
                EntityType::Projectile => {
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
                EntityType::ChargedShot => {
                    let charge_shot_bundle = ChargeShotBundle::new(tf.translation.truncate());
                    commands.spawn(charge_shot_bundle);

                    spaceship_shoot.state = ShootingState::Cooldown(Timer::from_seconds(
                        consts::PLAYER_CHARGE_SHOT_COOLDOWN,
                        TimerMode::Once,
                    ));
                }
                _ => (),
            },
            ShootingState::Cooldown(ref mut timer) => {
                timer.tick(time.delta());

                if timer.finished() {
                    spaceship_shoot.state = ShootingState::Idle;
                }
            }
        }
    }
}
