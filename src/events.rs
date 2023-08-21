use bevy::{prelude::*, window::WindowResized};
use rand::{thread_rng, Rng};

use crate::{
    common::{Asteroid, AsteroidType, EntityType},
    consts,
    enemy::EnemyCount,
    is_playing,
    movement::Velocity,
    player::{Invulnerability, Spaceship, SpaceshipHealth},
    Stats, WinSize,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnEntity>()
            .add_event::<AddScore>()
            .add_event::<SpaceshipIsHit>()
            .add_event::<SpawnEnemy>()
            .add_event::<SplitAsteroid>()
            .add_systems(
                PreUpdate,
                (
                    spaceship_hit_handler
                        .in_set(EventSet::HandleHit)
                        .after(EventSet::CreateEv),
                    despawn_entities_handler
                        .in_set(EventSet::HandleDespawn)
                        .after(EventSet::HandleHit),
                    add_score_handler
                        .in_set(EventSet::HandleScore)
                        .after(EventSet::HandleDespawn),
                    split_asteroid_handler
                        .in_set(EventSet::HandleAsteroidSplit)
                        .after(EventSet::HandleDespawn),
                    spawn_enemies_handler
                        .in_set(EventSet::HandleSpawn)
                        .after(EventSet::HandleScore),
                )
                    .run_if(is_playing),
            )
            .add_systems(PreUpdate, window_resize_handler);
    }
}

// ===

#[derive(Event)]
pub struct DespawnEntity {
    pub entity: Entity,
    pub entity_type: EntityType,
}

pub enum AddScoreType {
    EnemyDestroyed(EntityType),
}

#[derive(Event)]
pub struct AddScore(pub AddScoreType);

#[derive(Event)]
pub struct SpaceshipIsHit(pub Entity);

#[derive(Event)]
pub struct SplitAsteroid {
    translation: Vec3,
    size: Vec2,
    velocity: Velocity,
    asteroid: Asteroid,
}

impl SplitAsteroid {
    pub fn new(translation: Vec3, size: Vec2, velocity: Velocity, asteroid: Asteroid) -> Self {
        SplitAsteroid {
            translation,
            size,
            velocity,
            asteroid,
        }
    }
}

#[derive(Event)]
pub struct SpawnEnemy {
    entity_type: EntityType,
    initial_velocity: Velocity,
    spawn_point: Vec3,
}

impl SpawnEnemy {
    pub fn new(entity_type: EntityType, initial_velocity: Velocity, spawn_point: Vec3) -> Self {
        Self {
            entity_type,
            initial_velocity,
            spawn_point,
        }
    }
}

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub enum EventSet {
    CreateEv,
    HandleHit,
    HandleDespawn,
    HandleScore,
    HandleAsteroidSplit,
    HandleSpawn,
}

// ===

fn despawn_entities_handler(
    mut commands: Commands,
    mut ev_despawn: EventReader<DespawnEntity>,
    mut query: Query<&mut EnemyCount>,
) {
    if let Ok(mut enemy_count) = query.get_single_mut() {
        for despawn_ev in ev_despawn.iter() {
            commands.entity(despawn_ev.entity).despawn();

            enemy_count.remove_enemy_count(despawn_ev.entity_type, 1);
        }
    }
}

fn add_score_handler(mut add_score_events: EventReader<AddScore>, mut stats: ResMut<Stats>) {
    for add_score_ev in add_score_events.iter() {
        match add_score_ev.0 {
            AddScoreType::EnemyDestroyed(entity_type) => {
                if let EntityType::Asteroid(_) = entity_type {
                    stats.score += consts::SCORE_ADD_ASTEROID;
                }
            }
        }
    }
}

fn spaceship_hit_handler(
    mut commands: Commands,
    mut ev_hit: EventReader<SpaceshipIsHit>,
    mut ev_despawn: EventWriter<DespawnEntity>,
    mut spaceship_query: Query<&mut SpaceshipHealth>,
) {
    if let Ok(mut health) = spaceship_query.get_single_mut() {
        if let Some(hit_ev) = ev_hit.iter().next() {
            if health.0 > 0 {
                health.0 -= 1;

                if health.0 == 0 {
                    ev_despawn.send(DespawnEntity {
                        entity: hit_ev.0,
                        entity_type: EntityType::Spaceship,
                    });
                } else {
                    commands.entity(hit_ev.0).insert(Invulnerability::new());
                }
            }
        }
    }
}

fn window_resize_handler(
    mut win_size: ResMut<WinSize>,
    mut ev_resize: EventReader<WindowResized>,
    mut player_query: Query<&mut Transform, With<Spaceship>>,
) {
    // When window is resizing, look after which window dim
    // is the smaller value, then based on the smallest dim
    // calculate the other dim using the ratio
    // That will make the game always visible even when
    // the one dim is squashed
    for resize_ev in ev_resize.iter() {
        win_size.h = resize_ev.height;
        win_size.w = resize_ev.height * consts::WINDOW_RATIO;
    }

    // TODO ship can go outside screen bounds when resizing
    if let Ok(mut tf) = player_query.get_single_mut() {
        tf.translation.y = Spaceship::player_position(win_size.h);
    }
}

fn split_asteroid_handler(
    mut ev_spawn: EventWriter<SpawnEnemy>,
    mut ev_asteroid_split: EventReader<SplitAsteroid>,
) {
    for asteroid_split_ev in ev_asteroid_split.iter() {
        let asteroid = asteroid_split_ev.asteroid;
        let translation = asteroid_split_ev.translation;
        let velocity = asteroid_split_ev.velocity;
        let size = asteroid_split_ev.size;

        let mut rng = thread_rng();
        let entity_x = translation.x;
        let entity_y = translation.y;

        let w_span_left = entity_x - size.x / 2.0;
        let w_span_right = entity_x + size.x / 2.0;

        let h_span_bottom = entity_y - size.y / 2.0;
        let h_span_top = entity_y + size.y / 2.0;

        let asteroid_amount = rng.gen_range(0..2);
        for _ in 0..=asteroid_amount {
            let spawn_point = Vec3::new(
                rng.gen_range(w_span_left..w_span_right),
                rng.gen_range(h_span_bottom..h_span_top),
                consts::ENEMY_Z,
            );

            let asteroid_type = if let AsteroidType::Large = asteroid.asteroid_type {
                match rng.gen_bool(0.7) {
                    true => AsteroidType::Medium,
                    false => AsteroidType::Small,
                }
            } else {
                AsteroidType::Small
            };

            let initial_velocity = Velocity::new(rng.gen_range(-60.0..60.0), velocity.y);

            ev_spawn.send(SpawnEnemy::new(
                EntityType::Asteroid(Asteroid { asteroid_type }),
                initial_velocity,
                spawn_point,
            ));
        }
    }
}

fn spawn_enemies_handler(
    mut commands: Commands,
    mut ev_spawn: EventReader<SpawnEnemy>,
    mut query: Query<&mut EnemyCount>,
) {
    if let Ok(mut enemy_count) = query.get_single_mut() {
        for spawn_ev in ev_spawn.iter() {
            if let EntityType::Asteroid(asteroid) = spawn_ev.entity_type {
                let asteroid_bundle = asteroid.construct_asteroid_bundle(
                    spawn_ev.entity_type,
                    spawn_ev.initial_velocity,
                    spawn_ev.spawn_point,
                );
                commands.spawn(asteroid_bundle);
                enemy_count.add_enemy_count(spawn_ev.entity_type, 1);
            }
        }
    }
}
