use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use serde::{Deserialize, Serialize};

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const TILE_SIZE: f32 = 40.0;
const TILE_COLOR: Color = Color::rgb(0.5, 0.3, 0.2);

const PLAYER_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.1, 0.8, 0.1);
const PLAYER_SPEED: f32 = 100.0;
const PLAYER_GRAVITY: f32 = 3.0;

const CAMERA_FOCUS_OFFSET: f32 = -200.0;

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
        .add_systems(Startup, setup_player)
        .add_systems(Update, apply_velocity)
        .add_systems(Update, focus_camera_on_player)
        .add_systems(Update, player_gravity)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TileMap {
    map: Vec<Vec<u32>>,
}

#[derive(Component)]
struct TileGround;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

fn setup_camera(mut commands: Commands) {
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

                commands.spawn((
                    SpriteBundle {
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
                    },
                    TileGround,
                ));
            }
        }
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_x = -WINDOW_SIZE.x / 2.0 + PLAYER_SIZE.x;
    let player_y = WINDOW_SIZE.y / 5.0 - PLAYER_SIZE.y;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(1.0, 4).into()).into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            transform: Transform {
                translation: Vec3::new(player_x, player_y, 1.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            ..default()
        },
        Player,
        Velocity(Vec3::new(PLAYER_SPEED, 0.0, 0.0)),
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
    }
}

fn focus_camera_on_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    if player_transform.translation.x > CAMERA_FOCUS_OFFSET {
        camera_transform.translation.x = player_transform.translation.x - CAMERA_FOCUS_OFFSET;
    }
}

fn player_gravity(mut player_query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        player_transform.translation.y -= PLAYER_GRAVITY;
    }
}
