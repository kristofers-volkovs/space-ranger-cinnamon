use std::time::Duration;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

use crate::{
    common::{DespawnEntity, EntityType, EventSet},
    consts::{ENEMY_Z, SPAWN_MARGIN},
    is_playing,
    movement::{Movable, Velocity},
    player::{Spaceship, SpaceshipIsHit},
    WinSize,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyCount>()
            .init_resource::<EnemySpawner>()
            .insert_resource(FixedTime::new(Duration::from_secs(2)))
            .add_system(
                spawn_enemy
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .run_if(is_playing),
            )
            .add_system(
                enemy_collision_detection
                    .run_if(is_playing)
                    .in_set(EventSet::SpawnEvents),
            );
    }
}

// ===

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Resource, Default, Debug)]
pub struct EnemyCount {
    pub asteroids: u32,
}

enum EnemySpawnLocation {
    Top,
    // Sides,
}

#[derive(Resource)]
struct EnemySpawner {
    entity_type: EntityType,
    location: EnemySpawnLocation,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            entity_type: EntityType::Asteroid,
            location: EnemySpawnLocation::Top,
        }
    }
}

impl EnemySpawner {
    fn spawn_location(&self, win_size: &WinSize) -> Vec3 {
        let mut rng = thread_rng();

        let w_span = win_size.w / 2.0 + SPAWN_MARGIN;
        let h_span = win_size.h / 2.0 + SPAWN_MARGIN;

        match self.location {
            EnemySpawnLocation::Top => {
                let w_span = w_span - SPAWN_MARGIN;
                Vec3::new(rng.gen_range(-w_span..w_span), h_span, ENEMY_Z)
            }
            // EnemySpawnLocation::Sides => {
            //     Vec3::new(
            //         if rng.gen_bool(0.5) { w_span } else { -w_span },
            //         rng.gen_range(-h_span..h_span),
            //         ENEMY_Z
            //     )
            // },
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    entity_type: EntityType,
    movable: Movable,
    velocity: Velocity,
    #[bundle]
    sprite: SpriteBundle,
}

// ===

fn spawn_enemy(
    mut commands: Commands,
    win_size: Res<WinSize>,
    enemy_spawner: Res<EnemySpawner>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    let spawn_point = enemy_spawner.spawn_location(&win_size);

    commands.spawn(EnemyBundle {
        enemy: Enemy,
        entity_type: enemy_spawner.entity_type,
        movable: Movable { auto_despawn: true },
        velocity: Velocity { x: 0.0, y: -200.0 },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..default()
            },
            transform: Transform::from_translation(spawn_point),
            ..default()
        },
    });

    enemy_count.asteroids += 1;
}

fn enemy_collision_detection(
    mut ev_despawn: EventWriter<DespawnEntity>,
    mut ev_spaceship_hit: EventWriter<SpaceshipIsHit>,
    enemy_query: Query<(Entity, &Transform, &Sprite, &EntityType), With<Enemy>>,
    spaceship_query: Query<(Entity, &Transform, &Sprite), With<Spaceship>>,
) {
    if let Ok((spaceship_entity, spaceship_tf, spaceship_sprite)) = spaceship_query.get_single() {
        for (enemy_entity, enemy_tf, enemy_sprite, enemy_type) in enemy_query.iter() {
            let spaceship_size = {
                match spaceship_sprite.custom_size {
                    Some(size) => size * spaceship_tf.scale.xy(),
                    None => panic!("Spaceship sprite has no custom size"),
                }
            };
            let enemy_size = {
                match enemy_sprite.custom_size {
                    Some(size) => size * enemy_tf.scale.xy(),
                    None => panic!("Enemy sprite has no custom size"),
                }
            };

            let collision = collide(
                spaceship_tf.translation,
                spaceship_size,
                enemy_tf.translation,
                enemy_size,
            );

            if collision.is_some() {
                ev_spaceship_hit.send(SpaceshipIsHit {
                    entity: spaceship_entity,
                });
                ev_despawn.send(DespawnEntity {
                    entity: enemy_entity,
                    entity_type: *enemy_type,
                });
            }
        }
    }
}
