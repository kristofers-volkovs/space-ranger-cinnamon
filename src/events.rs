use bevy::{prelude::*, window::WindowResized};

use crate::{
    common::EntityType,
    consts,
    enemy::EnemyCount,
    is_playing,
    player::{Invulnerability, Spaceship, SpaceshipHealth},
    Stats, WinSize,
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
                if let EntityType::Asteroid(_) = entity_type {
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
    mut spaceship_query: Query<&mut SpaceshipHealth>,
) {
    if let Ok(mut health) = spaceship_query.get_single_mut() {
        if let Some(hit_ev) = hit_event.iter().next() {
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
