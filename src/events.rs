use bevy::prelude::*;

use crate::{
    common::EntityType,
    consts,
    enemy::EnemyCount,
    is_playing,
    player::{Invulnerability, SpaceshipState},
    Stats,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnEntity>()
            .add_event::<AddScore>()
            .add_event::<SpaceshipIsHit>()
            .add_systems(
                PreUpdate,
                (
                    spaceship_hit_handler
                        .in_set(EventSet::HandleHit)
                        .after(EventSet::Spawn),
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
    mut query: Query<&mut EnemyCount>,
) {
    if let Ok(mut enemy_count) = query.get_single_mut() {
        for despawn_ev in despawn_events.iter() {
            commands.entity(despawn_ev.entity).despawn();

            enemy_count.remove_enemy_count(despawn_ev.entity_type, 1);
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

fn spaceship_hit_handler(
    mut commands: Commands,
    mut hit_event: EventReader<SpaceshipIsHit>,
    mut ev_despawn: EventWriter<DespawnEntity>,
    mut spaceship_state: ResMut<SpaceshipState>,
) {
    if let Some(hit_ev) = hit_event.iter().next() {
        if spaceship_state.health > 0 {
            spaceship_state.health -= 1;

            if spaceship_state.health == 0 {
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
