use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};

use crate::{
    is_playing,
    player::{Projectile, ProjectileSource},
};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(hit_detection.run_if(is_playing));
    }
}

// ===

#[derive(Component, Clone, Copy, Debug)]
pub enum EntityType {
    Spaceship,
    Projectile,
    Asteroid,
}

// ===

fn hit_detection(
    mut commands: Commands,
    entity_query: Query<
        (Entity, &Transform, &Sprite, &EntityType),
        (With<EntityType>, Without<Projectile>),
    >,
    projectile_query: Query<(Entity, &Transform, &Sprite, &ProjectileSource), With<Projectile>>,
) {
    let mut processed_entities: HashSet<Entity> = HashSet::new();

    for (entity, entity_tf, entity_sprite, entity_type) in entity_query.iter() {
        if processed_entities.contains(&entity) {
            continue;
        }

        for (projectile, projectile_tf, projectile_sprite, projectile_source) in
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

            let entity_scale = entity_tf.scale.xy();
            let projectile_scale = projectile_tf.scale.xy();

            let projectile_size = {
                match projectile_sprite.custom_size {
                    Some(size) => size,
                    None => panic!("Projectile sprite has no custom size"),
                }
            };
            let entity_size = {
                match entity_sprite.custom_size {
                    Some(size) => size,
                    None => panic!("Entity sprite has no custom size"),
                }
            };

            let collision = collide(
                projectile_tf.translation,
                projectile_size * projectile_scale,
                entity_tf.translation,
                entity_size * entity_scale,
            );

            if collision.is_some() {
                commands.entity(projectile).despawn();
                processed_entities.insert(projectile);

                commands.entity(entity).despawn();
                processed_entities.insert(entity);
            }
        }
    }
}
