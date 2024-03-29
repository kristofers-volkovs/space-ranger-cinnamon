use bevy::{prelude::*, time::Stopwatch, window::WindowResolution};
use bevy_mod_aseprite::AsepritePlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod common;
mod consts;
mod enemy;
mod events;
mod movement;
mod player;
mod ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    LoadingGame,
    Gameplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameplayState {
    #[default]
    Playing,
    Paused,
    GameOver,
}

pub fn is_playing(game: Res<State<GameState>>, gameplay: Res<State<GameplayState>>) -> bool {
    matches!(game.get(), GameState::Gameplay) && matches!(gameplay.get(), GameplayState::Playing)
}

pub fn is_gameplay(game: Res<State<GameState>>) -> bool {
    matches!(game.get(), GameState::Gameplay)
}

fn main() {
    App::new()
        // --- Initial resources ---
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<WinSize>()
        .init_resource::<Stats>()
        .add_systems(Startup, load_font)
        // --- Initial game states ---
        .add_state::<GameState>()
        .add_state::<GameplayState>()
        // --- Install plugins ---
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            consts::WINDOW_WIDTH,
                            consts::WINDOW_HEIGHT,
                        ),
                        title: "Space Ranger Cinnamon".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
            camera::CameraPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            movement::MovementPlugin,
            common::CommonPlugin,
            ui::UiPlugin,
            events::EventsPlugin,
        ))
        .add_plugins(AsepritePlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}

// ===

#[derive(Resource, Debug)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

impl Default for WinSize {
    fn default() -> Self {
        Self {
            w: consts::WINDOW_WIDTH,
            h: consts::WINDOW_HEIGHT,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Stats {
    pub score: u32,
    pub watch: Stopwatch,
}

impl Default for Stats {
    fn default() -> Self {
        let mut watch = Stopwatch::new();
        watch.pause();
        Self { score: 0, watch }
    }
}

impl Stats {
    fn get_watch_time(&self) -> String {
        let elapsed_mins = (self.watch.elapsed_secs() / 60.0).floor();
        format!(
            "{:.0}:{:.0}",
            elapsed_mins,
            self.watch.elapsed_secs() - elapsed_mins * 60.0,
        )
    }
}

#[derive(Resource, Debug)]
pub struct FontHandle(Handle<Font>);

// ===

fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PixeloidSans-mLxMm.ttf");
    commands.insert_resource(FontHandle(font));
}

// Despawns all entities that have a specific component attached to it
pub fn despawn_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
