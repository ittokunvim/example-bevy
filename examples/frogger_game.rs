use std::f32::consts::PI;

use bevy::prelude::*;

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

const PLAYER_INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, BOARD_SIZE_J as f32 / 2.0 - 0.5);

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            PLAYER_INITIAL_POSITION.x - 2.8,
            PLAYER_INITIAL_POSITION.y + 3.0,
            PLAYER_INITIAL_POSITION.z + 3.5,
        )
        .looking_at(Vec3::from(PLAYER_INITIAL_POSITION), Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            range: 30.0,
            ..default()
        },
        ..default()
    });

    // Board
    let cell_scene = asset_server.load("models/Frogger/tile.glb#Scene0");

    (0..BOARD_SIZE_I).for_each(|i| {
        (0..BOARD_SIZE_J).for_each(|j| {
            commands.spawn(SceneBundle {
                transform: Transform::from_xyz(i as f32, -0.2, j as f32),
                scene: cell_scene.clone(),
                ..default()
            });
        });
    });

    // Player
    let player_asset = asset_server.load("models/Frogger/gekota.glb#Scene0");

    commands.spawn(SceneBundle {
        transform: Transform {
            translation: PLAYER_INITIAL_POSITION,
            rotation: Quat::from_rotation_y(-PI / 2.),
            ..default()
        },
        scene: player_asset,
        ..default()
    });
}
