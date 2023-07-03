use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{EngineState, InGameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(
                spawn_player
                    .run_if(in_state(EngineState::InGame))
                    .run_if(in_state(InGameState::Playing)),
            );
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Move,
    Dash,
    Shoot,
    ChargeShot,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2 { x: 50., y: 50. }),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        }
    ));
}
