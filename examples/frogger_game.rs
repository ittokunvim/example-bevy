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
        .add_system(move_player)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player {
    i: usize,
    j: usize,
    move_cooldown: Timer,
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

    commands.spawn((
        SceneBundle {
            transform: Transform {
                translation: PLAYER_INITIAL_POSITION,
                rotation: Quat::from_rotation_y(PI / 2.0),
                ..default()
            },
            scene: player_asset,
            ..default()
        },
        Player {
            i: PLAYER_INITIAL_POSITION.x as usize,
            j: PLAYER_INITIAL_POSITION.z as usize,
            move_cooldown: Timer::from_seconds(0.3, TimerMode::Once),
        },
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();

    if player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        let mut rotation = 0.0;

        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            if player.i < BOARD_SIZE_I - 1 {
                player.i += 1;
            }
            rotation = PI / 2.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            if player.i > 0 {
                player.i -= 1;
            }
            rotation = -PI / 2.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            if player.j < BOARD_SIZE_J - 1 {
                player.j += 1;
            }
            rotation = 0.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            if player.j > 0 {
                player.j -= 1;
            }
            rotation = PI;
            moved = true;
        }

        if moved {
            player.move_cooldown.reset();
            player_transform.translation = Vec3::new(player.i as f32, 0.0, player.j as f32);
            player_transform.rotation = Quat::from_rotation_y(rotation);
        }
    }
}
