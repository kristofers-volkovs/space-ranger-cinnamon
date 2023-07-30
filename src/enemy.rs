use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

use crate::{
    common::EntityType,
    consts,
    events::{DespawnEntity, EventSet, SpaceshipIsHit},
    is_playing,
    movement::{Movable, Velocity},
    player::{Invulnerability, Point, Spaceship},
    GameState, WinSize,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_stage)
            .add_systems(
                PreUpdate,
                (out_of_bounds_detection, enemy_collision_detection)
                    .run_if(is_playing)
                    .in_set(EventSet::Spawn),
            )
            .add_systems(PreUpdate, stage_manager.run_if(is_playing));
    }
}

// ===

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Debug)]
struct SpawnerLocation {
    center: Point,
    width: f32,
    height: f32,
}

#[derive(Component, Debug)]
struct EnemySpawner {
    entity_type: EntityType,
    spawned: u32,
    spawn_total: u32,
    interval: Timer,
    location: SpawnerLocation,
}

impl EnemySpawner {
    fn init_velocity(&self) -> Velocity {
        match self.entity_type {
            EntityType::Asteroid => Velocity { x: 0.0, y: -200.0 },
            _ => Velocity { x: 0.0, y: 0.0 },
        }
    }

    fn entity_sprite(&self) -> Sprite {
        match self.entity_type {
            EntityType::Asteroid => Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..default()
            },
            _ => Sprite {
                color: Color::rgb(0.5, 1.0, 0.5),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
        }
    }

    fn spawn_location(&self) -> Vec3 {
        let mut rng = thread_rng();

        let w_span_left = self.location.center.x - self.location.width / 2.0;
        let w_span_right = self.location.center.x + self.location.width / 2.0;

        let h_span_bottom = self.location.center.y - self.location.height / 2.0;
        let h_span_top = self.location.center.y + self.location.height / 2.0;

        Vec3::new(
            rng.gen_range(w_span_left..w_span_right),
            rng.gen_range(h_span_bottom..h_span_top),
            consts::ENEMY_Z,
        )
    }
}

#[derive(Component, Debug)]
enum StageState {
    Spawning(Vec<EnemySpawner>),
    Cooldown(Timer),
}

#[derive(Component, Debug)]
struct StageWave(u32);

#[derive(Component, Debug)]
struct GameplayStage {
    wave: StageWave,
    state: StageState,
}

#[derive(Component)]
pub struct EnemyCount {
    pub asteroids: u32,
}

impl EnemyCount {
    pub fn add_enemy_count(&mut self, entity_type: EntityType, amount: u32) {
        if matches!(entity_type, EntityType::Asteroid) {
            self.asteroids += amount;
        }
    }

    pub fn remove_enemy_count(&mut self, entity_type: EntityType, amount: u32) {
        if matches!(entity_type, EntityType::Asteroid) {
            self.asteroids -= amount;
        }
    }
}

#[derive(Bundle)]
struct GameplayBundle {
    stage: GameplayStage,
    enemy_count: EnemyCount,
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    entity_type: EntityType,
    movable: Movable,
    velocity: Velocity,
    #[bundle()]
    sprite: SpriteBundle,
}

// ===

fn spawn_stage(mut commands: Commands) {
    commands.spawn(GameplayBundle {
        stage: GameplayStage {
            wave: StageWave(0),
            state: StageState::Cooldown(Timer::from_seconds(
                consts::STAGE_INIT_COOLDOWN,
                TimerMode::Once,
            )),
        },
        enemy_count: EnemyCount { asteroids: 0 },
    });
}

fn stage_manager(
    mut commands: Commands,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<(&mut GameplayStage, &mut EnemyCount)>,
) {
    if let Ok((mut stage, mut enemy_count)) = query.get_single_mut() {
        match stage.state {
            StageState::Spawning(ref mut spawners) => {
                let mut finished_spawners = vec![];

                for (idx, mut spawner) in spawners.iter_mut().enumerate() {
                    if spawner.spawned >= spawner.spawn_total {
                        finished_spawners.push(idx);
                        continue;
                    }

                    spawner.interval.tick(time.delta());

                    if spawner.interval.finished() {
                        let velocity = spawner.init_velocity();
                        let sprite = spawner.entity_sprite();
                        let spawn = spawner.spawn_location();

                        commands.spawn(EnemyBundle {
                            enemy: Enemy,
                            entity_type: spawner.entity_type,
                            movable: Movable { auto_despawn: true },
                            velocity,
                            sprite: SpriteBundle {
                                sprite,
                                transform: Transform::from_translation(spawn),
                                ..default()
                            },
                        });

                        spawner.spawned += 1;
                        enemy_count.add_enemy_count(spawner.entity_type, 1);
                    }
                }

                // Remove spawners that have finished
                for idx in finished_spawners.iter().rev() {
                    spawners.remove(*idx);
                }

                // When spawners have finished start cooldown
                if spawners.is_empty() {
                    stage.state = StageState::Cooldown(Timer::from_seconds(
                        consts::STAGE_COOLDOWN,
                        TimerMode::Once,
                    ));
                }
            }
            StageState::Cooldown(ref mut timer) => {
                timer.tick(time.delta());

                if timer.finished() {
                    stage.wave.0 += 1;
                    // TODO add variation to different waves
                    // Different events - asteroid field, saucer invasion, etc.
                    // Different difficulty levels
                    // Variation to spawner locations and intervals

                    let spawn_total = 60;
                    let interval = consts::STAGE_LENGTH / spawn_total as f32;
                    let spawner_location = SpawnerLocation {
                        center: Point {
                            x: 0.0,
                            y: win_size.h / 2.0 + consts::SPAWN_MARGIN,
                        },
                        width: win_size.w - consts::SPAWN_MARGIN * 2.0,
                        height: 10.0,
                    };

                    let spawners = vec![EnemySpawner {
                        entity_type: EntityType::Asteroid,
                        spawned: 0,
                        spawn_total,
                        interval: Timer::from_seconds(interval, TimerMode::Repeating),
                        location: spawner_location,
                    }];

                    stage.state = StageState::Spawning(spawners);
                }
            }
        }
    }
}

fn enemy_collision_detection(
    mut ev_despawn: EventWriter<DespawnEntity>,
    mut ev_spaceship_hit: EventWriter<SpaceshipIsHit>,
    enemy_query: Query<(Entity, &Transform, &Sprite, &EntityType), With<Enemy>>,
    spaceship_query: Query<
        (Entity, &Transform, &Sprite),
        (With<Spaceship>, Without<Invulnerability>),
    >,
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
                ev_spaceship_hit.send(SpaceshipIsHit(spaceship_entity));
                ev_despawn.send(DespawnEntity {
                    entity: enemy_entity,
                    entity_type: *enemy_type,
                });
            }
        }
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
            let h_bound = win_size.h / 2.0 + consts::DESPAWN_MARGIN;
            let w_bound = win_size.w / 2.0 + consts::DESPAWN_MARGIN;

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
