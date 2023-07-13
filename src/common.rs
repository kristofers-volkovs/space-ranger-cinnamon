use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};

use crate::{
    enemy::EnemyCount,
    is_playing,
    player::{Invulnerability, Projectile, ProjectileSource},
};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnEntity>().add_systems(
            PreUpdate,
            (
                projectile_hit_detection.in_set(EventSet::Spawn),
                despawn_entities_handler
                    .in_set(EventSet::HandleDespawn)
                    .after(EventSet::HandleHit),
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

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub enum EventSet {
    Spawn,
    HandleHit,
    HandleDespawn,
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

fn projectile_hit_detection(
    mut ev_despawn: EventWriter<DespawnEntity>,
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

                processed_entities.insert(projectile);
                ev_despawn.send(DespawnEntity {
                    entity: projectile,
                    entity_type: *projectile_type,
                });
            }
        }
    }
}
