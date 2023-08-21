use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

use crate::{
    common::{Asteroid, AsteroidType, EntityType},
    consts,
    events::{DespawnEntity, EventSet, SpaceshipIsHit, SpawnEnemy},
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
                    .in_set(EventSet::CreateEv),
            )
            .add_systems(PreUpdate, stage_manager.run_if(is_playing));
    }
}

// ===

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Debug, Copy, Clone)]
struct SpawnerArea {
    center: Point,
    width: f32,
    height: f32,
}

#[derive(Component, Debug)]
struct EnemySpawner {
    entity_type: EntityType,
    // TODO Each tick a random amount of enemies spawn equaling up to a predetermined total amount
    // spawn_per_tick: Vec2,
    // tick: Timer,
    spawned: u32,
    spawn_total: u32,
    interval: Timer,
    area: SpawnerArea,
}

impl EnemySpawner {
    fn get_enemy_spawn_point(&self) -> Vec3 {
        let mut rng = thread_rng();

        let w_span_left = self.area.center.x - self.area.width / 2.0;
        let w_span_right = self.area.center.x + self.area.width / 2.0;

        let h_span_bottom = self.area.center.y - self.area.height / 2.0;
        let h_span_top = self.area.center.y + self.area.height / 2.0;

        Vec3::new(
            rng.gen_range(w_span_left..w_span_right),
            rng.gen_range(h_span_bottom..h_span_top),
            consts::ENEMY_Z,
        )
    }

    fn get_enemy_initial_velocity(&self) -> Velocity {
        if let EntityType::Asteroid(asteroid) = self.entity_type {
            match asteroid.asteroid_type {
                AsteroidType::Small => Velocity::new(0.0, -300.0),
                AsteroidType::Medium => Velocity::new(0.0, -200.0),
                AsteroidType::Large => Velocity::new(0.0, -100.0),
            }
        } else {
            Velocity { x: 0.0, y: 0.0 }
        }
    }

    fn add_spawned_count(&mut self, amount: u32) {
        self.spawned += amount;
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
            let spawn_total = 10 * wave;
            let interval = consts::STAGE_LENGTH / spawn_total as f32;
            let spawner_location = SpawnerArea {
                center: Point {
                    x: 0.0,
                    y: win_size.h / 2.0 + consts::SPAWN_MARGIN,
                },
                width: win_size.w - consts::SPAWN_MARGIN * 2.0,
                height: 30.0,
            };

            vec![
                EnemySpawner {
                    entity_type: EntityType::Asteroid(Asteroid {
                        asteroid_type: AsteroidType::Small,
                    }),
                    spawned: 0,
                    spawn_total,
                    interval: Timer::from_seconds(interval, TimerMode::Repeating),
                    area: spawner_location,
                },
                EnemySpawner {
                    entity_type: EntityType::Asteroid(Asteroid {
                        asteroid_type: AsteroidType::Medium,
                    }),
                    spawned: 0,
                    spawn_total,
                    interval: Timer::from_seconds(interval, TimerMode::Repeating),
                    area: spawner_location,
                },
                EnemySpawner {
                    entity_type: EntityType::Asteroid(Asteroid {
                        asteroid_type: AsteroidType::Large,
                    }),
                    spawned: 0,
                    spawn_total,
                    interval: Timer::from_seconds(interval, TimerMode::Repeating),
                    area: spawner_location,
                },
            ]
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
        if matches!(entity_type, EntityType::Asteroid(_)) {
            self.asteroids += amount;
        }
    }

    pub fn remove_enemy_count(&mut self, entity_type: EntityType, amount: u32) {
        if matches!(entity_type, EntityType::Asteroid(_)) {
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
pub struct EnemyBundle {
    enemy: Enemy,
    entity_type: EntityType,
    movable: Movable,
    velocity: Velocity,
    #[bundle()]
    sprite: SpriteBundle,
}

impl EnemyBundle {
    // implement new method that would initiate all the fields from passed parameters
    // and return the bundle
    pub fn new(
        entity_type: EntityType,
        velocity: Velocity,
        sprite: Sprite,
        spawn_point: Vec3,
    ) -> Self {
        EnemyBundle {
            enemy: Enemy,
            entity_type,
            movable: Movable::new(true),
            velocity,
            sprite: SpriteBundle {
                sprite,
                transform: Transform::from_translation(spawn_point),
                ..default()
            },
        }
    }
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
    mut ev_spawn: EventWriter<SpawnEnemy>,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<&mut GameplayStage>,
) {
    if let Ok(mut stage) = query.get_single_mut() {
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
                        let spawn_point = spawner.get_enemy_spawn_point();
                        let initial_velocity = spawner.get_enemy_initial_velocity();

                        ev_spawn.send(SpawnEnemy::new(
                            spawner.entity_type,
                            initial_velocity,
                            spawn_point,
                        ));
                        spawner.add_spawned_count(1);
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
            let enemy_size = match enemy_sprite.custom_size {
                Some(size) => size * enemy_tf.scale.xy(),
                None => panic!("Enemy sprite has no custom size"),
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
