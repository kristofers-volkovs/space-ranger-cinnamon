use std::time::Duration;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    consts::{ENEMY_Z, SPAWN_MARGIN},
    movement::{Movable, Velocity},
    WinSize,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyCount>()
            .init_resource::<EnemySpawner>()
            .insert_resource(FixedTime::new(Duration::from_secs(2)))
            .add_system(spawn_enemy.in_schedule(CoreSchedule::FixedUpdate));
    }
}

// ===

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Resource, Default, Debug)]
pub struct EnemyCount {
    pub asteroids: u32,
}

#[derive(Component, Clone, Copy, Debug)]
enum EnemyType {
    Asteroid,
}

enum EnemySpawnLocation {
    Top,
    // Sides,
}

#[derive(Resource)]
struct EnemySpawner {
    enemy_type: EnemyType,
    location: EnemySpawnLocation,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            enemy_type: EnemyType::Asteroid,
            location: EnemySpawnLocation::Top,
        }
    }
}

impl EnemySpawner {
    fn spawn_location(&self, win_size: &WinSize) -> Vec3 {
        let mut rng = thread_rng();

        let w_span = win_size.w / 2.0 + SPAWN_MARGIN;
        let h_span = win_size.h / 2.0 + SPAWN_MARGIN;

        match self.location {
            EnemySpawnLocation::Top => {
                let w_span = w_span - SPAWN_MARGIN;
                Vec3::new(rng.gen_range(-w_span..w_span), h_span, ENEMY_Z)
            }
            // EnemySpawnLocation::Sides => {
            //     Vec3::new(
            //         if rng.gen_bool(0.5) { w_span } else { -w_span },
            //         rng.gen_range(-h_span..h_span),
            //         ENEMY_Z
            //     )
            // },
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    enemy_type: EnemyType,
    movable: Movable,
    velocity: Velocity,
    #[bundle]
    sprite: SpriteBundle,
}

// ===

fn spawn_enemy(
    mut commands: Commands,
    win_size: Res<WinSize>,
    enemy_spawner: Res<EnemySpawner>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    let spawn_point = enemy_spawner.spawn_location(&win_size);

    commands.spawn(EnemyBundle {
        enemy: Enemy,
        enemy_type: enemy_spawner.enemy_type,
        movable: Movable { auto_despawn: true },
        velocity: Velocity { x: 0.0, y: -200.0 },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..default()
            },
            transform: Transform::from_translation(spawn_point),
            ..default()
        },
    });

    enemy_count.asteroids += 1;
}
