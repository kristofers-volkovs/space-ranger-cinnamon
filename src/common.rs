use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};

use crate::{
    consts,
    enemy::EnemyCount,
    is_playing,
    player::{Invulnerability, Projectile, ProjectileSource},
    Stats,
};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnEntity>()
            .add_event::<AddScore>()
            .add_systems(
                PreUpdate,
                (
                    projectile_hit_detection.in_set(EventSet::Spawn),
                    despawn_entities_handler
                        .in_set(EventSet::HandleDespawn)
                        .after(EventSet::HandleHit),
                    add_score_handler
                        .in_set(EventSet::HandleScore)
                        .after(EventSet::HandleDespawn),
                )
                    .run_if(is_playing),
            );
    }
}

// ===

#[derive(Component, Clone, Copy, Debug)]
pub enum EntityType {
    Spaceship,
    Projectile,
    Asteroid,
}

#[derive(Event)]
pub struct DespawnEntity {
    pub entity: Entity,
    pub entity_type: EntityType,
}

pub enum AddScoreType {
    EnemyDestroyed(EntityType),
}

#[derive(Event)]
pub struct AddScore(AddScoreType);

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub enum EventSet {
    Spawn,
    HandleHit,
    HandleDespawn,
    HandleScore,
}

// ===

fn despawn_entities_handler(
    mut commands: Commands,
    mut despawn_events: EventReader<DespawnEntity>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    for despawn_ev in despawn_events.iter() {
        commands.entity(despawn_ev.entity).despawn();

        if matches!(despawn_ev.entity_type, EntityType::Asteroid) {
            enemy_count.asteroids -= 1;
        }
    }
}

fn add_score_handler(mut add_score_events: EventReader<AddScore>, mut stats: ResMut<Stats>) {
    for add_score_ev in add_score_events.iter() {
        match add_score_ev.0 {
            AddScoreType::EnemyDestroyed(entity_type) => {
                if let EntityType::Asteroid = entity_type {
                    stats.score += consts::SCORE_ADD_ASTEROID;
                }
            }
        }
    }
}

// Despawns all entities that have a specific component attached to it
pub fn despawn_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn projectile_hit_detection(
    mut ev_despawn: EventWriter<DespawnEntity>,
    mut ev_add_score: EventWriter<AddScore>,
    entity_query: Query<
        (Entity, &Transform, &Sprite, &EntityType),
        (
            With<EntityType>,
            Without<Projectile>,
            Without<Invulnerability>,
        ),
    >,
    projectile_query: Query<
        (Entity, &Transform, &Sprite, &ProjectileSource, &EntityType),
        With<Projectile>,
    >,
) {
    let mut processed_entities: HashSet<Entity> = HashSet::new();

    for (entity, entity_tf, entity_sprite, entity_type) in entity_query.iter() {
        if processed_entities.contains(&entity) {
            continue;
        }

        for (projectile, projectile_tf, projectile_sprite, projectile_source, projectile_type) in
            projectile_query.iter()
        {
            if matches!(entity_type, EntityType::Spaceship)
                && matches!(projectile_source, ProjectileSource::FromSpaceship)
            {
                continue;
            }

            if processed_entities.contains(&entity) || processed_entities.contains(&projectile) {
                continue;
            }

            let projectile_size = {
                match projectile_sprite.custom_size {
                    Some(size) => size * entity_tf.scale.xy(),
                    None => panic!("Projectile sprite has no custom size"),
                }
            };
            let entity_size = {
                match entity_sprite.custom_size {
                    Some(size) => size * projectile_tf.scale.xy(),
                    None => panic!("Entity sprite has no custom size"),
                }
            };

            let collision = collide(
                projectile_tf.translation,
                projectile_size,
                entity_tf.translation,
                entity_size,
            );

            if collision.is_some() {
                processed_entities.insert(entity);
                ev_despawn.send(DespawnEntity {
                    entity,
                    entity_type: *entity_type,
                });

                if matches!(projectile_source, ProjectileSource::FromSpaceship) {
                    ev_add_score.send(AddScore(AddScoreType::EnemyDestroyed(*entity_type)));
                }

                processed_entities.insert(projectile);
                ev_despawn.send(DespawnEntity {
                    entity: projectile,
                    entity_type: *projectile_type,
                });
            }
        }
    }
}
