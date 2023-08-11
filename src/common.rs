use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};

use crate::{
    events::{AddScore, AddScoreType, DespawnEntity, EventSet},
    is_playing,
    player::{Invulnerability, PlayerAssetDimensions, Projectile, ProjectileSource}, movement::Velocity,
};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            projectile_hit_detection
                .in_set(EventSet::Spawn)
                .run_if(is_playing),
        );
    }
}

// ===

#[derive(Clone, Copy, Debug)]
pub enum AsteroidType {
    Small,
    Medium,
    Large,
}

impl AsteroidType {
    pub fn get_type_velocity(asteroid_type: AsteroidType) -> Velocity {
        match asteroid_type {
            AsteroidType::Small => Velocity { x: 0.0, y: -300.0 },
            AsteroidType::Medium => Velocity { x: 0.0, y: -200.0 },
            AsteroidType::Large => Velocity { x: 0.0, y: -100.0 },
        }
    }

    pub fn get_type_sprite(asteroid_type: AsteroidType) -> Sprite {
        Sprite {
            color: Color::rgb(0.5, 0.5, 0.5),
            custom_size: match asteroid_type {
                AsteroidType::Small => Some(Vec2::new(20.0, 20.0)),
                AsteroidType::Medium => Some(Vec2::new(40.0, 40.0)),
                AsteroidType::Large => Some(Vec2::new(70.0, 70.0)),
            },
            ..default()
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub enum EntityType {
    Spaceship,
    Projectile,
    Asteroid(AsteroidType),
}

// ===

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
    projectile_query: Query<(Entity, &Transform, &ProjectileSource, &EntityType), With<Projectile>>,
    player_asset_dimensions: Res<PlayerAssetDimensions>,
) {
    let mut processed_entities: HashSet<Entity> = HashSet::new();

    for (entity, entity_tf, entity_sprite, entity_type) in entity_query.iter() {
        if processed_entities.contains(&entity) {
            continue;
        }

        for (projectile, projectile_tf, projectile_source, projectile_type) in
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

            let entity_size = {
                match entity_sprite.custom_size {
                    Some(size) => size * projectile_tf.scale.xy(),
                    None => player_asset_dimensions.spaceship,
                }
            };

            let collision = collide(
                projectile_tf.translation,
                player_asset_dimensions.projectile,
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
