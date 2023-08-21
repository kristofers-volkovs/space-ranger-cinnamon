use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};

use crate::{
    enemy::EnemyBundle,
    events::{AddScore, AddScoreType, DespawnEntity, EventSet, SplitAsteroid},
    is_playing,
    movement::{Movable, Velocity},
    player::{Invulnerability, PlayerAssetDimensions},
};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            projectile_hit_detection
                .in_set(EventSet::CreateEv)
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

#[derive(Clone, Copy, Debug)]
pub struct Asteroid {
    pub asteroid_type: AsteroidType,
}

impl Asteroid {
    pub fn construct_asteroid_bundle(
        &self,
        entity_type: EntityType,
        initial_velocity: Velocity,
        spawn_point: Vec3,
    ) -> EnemyBundle {
        let sprite = Sprite {
            color: Color::rgb(0.5, 0.5, 0.5),
            custom_size: match self.asteroid_type {
                AsteroidType::Small => Some(Vec2::new(20.0, 20.0)),
                AsteroidType::Medium => Some(Vec2::new(40.0, 40.0)),
                AsteroidType::Large => Some(Vec2::new(70.0, 70.0)),
            },
            ..default()
        };

        EnemyBundle::new(entity_type, initial_velocity, sprite, spawn_point)
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub enum EntityType {
    Spaceship,
    Projectile,
    ChargedShot,
    Asteroid(Asteroid),
}

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Component, Debug)]
pub enum ProjectileSource {
    FromSpaceship,
    // FromEnemy,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    entity_type: EntityType,
    velocity: Velocity,
    movable: Movable,
    source: ProjectileSource,
    #[bundle()]
    sprite: SpriteBundle,
}

impl ProjectileBundle {
    pub fn new(
        entity_type: EntityType,
        velocity: Velocity,
        spawn_point: Vec3,
        texture: Handle<Image>,
        source: ProjectileSource,
    ) -> Self {
        Self {
            projectile: Projectile,
            entity_type,
            velocity,
            movable: Movable::new(true),
            source,
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_translation(spawn_point),
                ..default()
            },
        }
    }
}

// ===

fn projectile_hit_detection(
    mut ev_despawn: EventWriter<DespawnEntity>,
    mut ev_add_score: EventWriter<AddScore>,
    mut ev_split_asteroid: EventWriter<SplitAsteroid>,
    entity_query: Query<
        (Entity, &Transform, &Sprite, &EntityType, &Velocity),
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

    for (entity, entity_tf, entity_sprite, entity_type, velocity) in entity_query.iter() {
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

                if let EntityType::Asteroid(asteroid) = entity_type {
                    if !matches!(asteroid.asteroid_type, AsteroidType::Small) {
                        ev_split_asteroid.send(SplitAsteroid::new(
                            entity_tf.translation,
                            entity_size,
                            *velocity,
                            *asteroid,
                        ));
                    }
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
