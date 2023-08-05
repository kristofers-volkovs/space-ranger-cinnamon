use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

use crate::{
    common::EntityType,
    consts,
    events::{DespawnEntity, EventSet, SpaceshipIsHit},
    is_playing,
    movement::{Movable, Velocity},
    player::{Invulnerability, PlayerAssetDimensions, Point, Spaceship},
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
    fn spawn_enemy(&mut self, commands: &mut Commands) {
        let velocity = match self.entity_type {
            EntityType::Asteroid => Velocity { x: 0.0, y: -200.0 },
            _ => Velocity { x: 0.0, y: 0.0 },
        };

        let sprite = match self.entity_type {
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
        };

        let spawn = {
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
        };

        commands.spawn(EnemyBundle {
            enemy: Enemy,
            entity_type: self.entity_type,
            movable: Movable { auto_despawn: true },
            velocity,
            sprite: SpriteBundle {
                sprite,
                transform: Transform::from_translation(spawn),
                ..default()
            },
        });
        self.spawned += 1;
    }

    fn create_spawners(
        wave: &u32,
        stage_type: &StageType,
        win_size: &WinSize,
    ) -> Vec<EnemySpawner> {
        // TODO add variation to different waves
        // Different events - asteroid field, saucer invasion, etc.
        // Different difficulty levels
        // Variation to spawner locations and intervals

        if matches!(stage_type, StageType::Normal) {
            let spawn_total = 20 * wave;
            let interval = consts::STAGE_LENGTH / spawn_total as f32;
            let spawner_location = SpawnerLocation {
                center: Point {
                    x: 0.0,
                    y: win_size.h / 2.0 + consts::SPAWN_MARGIN,
                },
                width: win_size.w - consts::SPAWN_MARGIN * 2.0,
                height: 30.0,
            };

            vec![EnemySpawner {
                entity_type: EntityType::Asteroid,
                spawned: 0,
                spawn_total,
                interval: Timer::from_seconds(interval, TimerMode::Repeating),
                location: spawner_location,
            }]
        } else {
            vec![]
        }
    }
}

#[derive(Component, Debug)]
enum StageState {
    Spawning(Vec<EnemySpawner>),
    Cooldown(Timer),
}

#[derive(Debug)]
enum StageType {
    Normal,
    // AsteroidField,
    // SaucerInvasion,
}

#[derive(Component, Debug)]
struct StageWave {
    wave: u32,
    stage_type: StageType,
}

impl StageWave {
    fn new() -> StageWave {
        StageWave {
            wave: 0,
            stage_type: StageType::Normal,
        }
    }

    fn next_wave(&mut self) {
        self.wave += 1;

        // let mut rng = thread_rng();
        // if rng.gen_ratio(3, 10) {
        //     self.stage_type = StageType::AsteroidField;
        // } else if !matches!(self.stage_type, StageType::Normal) {
        //     self.stage_type = StageType::Normal;
        // }
    }
}

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
    pub fn new() -> EnemyCount {
        EnemyCount { asteroids: 0 }
    }

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
            wave: StageWave::new(),
            state: StageState::Cooldown(Timer::from_seconds(
                consts::STAGE_INIT_COOLDOWN,
                TimerMode::Once,
            )),
        },
        enemy_count: EnemyCount::new(),
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

                for (idx, spawner) in spawners.iter_mut().enumerate() {
                    if spawner.spawned >= spawner.spawn_total {
                        finished_spawners.push(idx);
                        continue;
                    }

                    spawner.interval.tick(time.delta());

                    if spawner.interval.finished() {
                        spawner.spawn_enemy(&mut commands);
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
                    stage.wave.next_wave();
                    let spawners = EnemySpawner::create_spawners(
                        &stage.wave.wave,
                        &stage.wave.stage_type,
                        &win_size,
                    );
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
    spaceship_query: Query<(Entity, &Transform), (With<Spaceship>, Without<Invulnerability>)>,
    player_asset_dimensions: Res<PlayerAssetDimensions>,
) {
    if let Ok((spaceship_entity, spaceship_tf)) = spaceship_query.get_single() {
        for (enemy_entity, enemy_tf, enemy_sprite, enemy_type) in enemy_query.iter() {
            let enemy_size = {
                match enemy_sprite.custom_size {
                    Some(size) => size * enemy_tf.scale.xy(),
                    None => panic!("Enemy sprite has no custom size"),
                }
            };

            let collision = collide(
                spaceship_tf.translation,
                player_asset_dimensions.spaceship,
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
