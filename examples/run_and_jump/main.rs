use bevy::prelude::*;

use serde::{Deserialize, Serialize};

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const TILE_SIZE: f32 = 40.0;
const TILE_COLOR: Color = Color::rgb(0.5, 0.3, 0.2);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_tilemap)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TileMap {
    map: Vec<Vec<u32>>,
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_tilemap(mut commands: Commands) {
    let tile_map: TileMap = serde_json::from_slice(include_bytes!("map.json")).unwrap();
    let window_top_left = Vec2::new(-WINDOW_SIZE.x / 2.0, WINDOW_SIZE.y / 2.0);

    for (y, row) in tile_map.map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &1 {
                let tile_y = window_top_left.y - TILE_SIZE * (y as f32 + 0.5) as f32;
                let tile_x = window_top_left.x + TILE_SIZE * (x as f32 + 0.5) as f32;

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: TILE_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(tile_x, tile_y, 0.0),
                        scale: Vec3::splat(TILE_SIZE),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}
