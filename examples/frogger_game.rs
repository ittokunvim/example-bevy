use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

const BOARD_SIZE_I: usize = 12;
const BOARD_SIZE_J: usize = 8;

const CAMERA_SPEED: f32 = 2.0;
const CAMERA_DISTANCE: Vec3 = Vec3::new(-2.8, 3.0, 3.5);

const PLAYER_INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, BOARD_SIZE_J as f32 / 2.0 - 0.5);

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const SCORE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .insert_resource(Scoreboard { score: 0 })
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(focus_camera)
        .add_system(goal_player)
        .add_system(apply_velocity)
        .add_system(check_for_collision)
        .add_system(move_obstacle)
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Camera {
    looking_at: Vec3,
}

#[derive(Component)]
struct Player {
    i: f32,
    j: f32,
    move_cooldown: Timer,
}

#[derive(Component)]
struct Obstacle {
    i: f32,
    j: f32,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Resource)]
struct Scoreboard {
    score: isize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(PLAYER_INITIAL_POSITION + CAMERA_DISTANCE)
                .looking_at(Vec3::from(PLAYER_INITIAL_POSITION), Vec3::Y),
            ..default()
        },
        Camera {
            looking_at: Vec3::from(PLAYER_INITIAL_POSITION),
        },
    ));

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
            i: PLAYER_INITIAL_POSITION.x,
            j: PLAYER_INITIAL_POSITION.z,
            move_cooldown: Timer::from_seconds(0.3, TimerMode::Once),
        },
    ));

    // Obstacles
    for i in 1..BOARD_SIZE_I - 1 {
        if i % 2 == 0 {
            continue;
        }

        let transform_z = rand::thread_rng().gen_range(0..BOARD_SIZE_J) as f32;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(i as f32, 0.5, transform_z),
                ..default()
            },
            Obstacle {
                i: i as f32,
                j: transform_z,
            },
            Velocity(Vec3::new(0.0, 0.0, i as f32)),
        ));
    }

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
}

fn apply_velocity(
    mut query: Query<(&mut Obstacle, &mut Transform, &Velocity)>,
    time_step: Res<FixedTime>,
) {
    for (mut obstacle, mut transform, velocity) in &mut query {
        obstacle.j = transform.translation.z;
        transform.translation.z += velocity.z * time_step.period.as_secs_f32();
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn check_for_collision(
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    obstacle_query: Query<&Obstacle, With<Obstacle>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    let player_j = (player.j as f32 - 0.5, player.j as f32 + 0.5);

    for obstacle in &obstacle_query {
        let obstacle_j = (obstacle.j as f32 - 0.5, obstacle.j as f32 + 0.5);

        if player_j.0 < obstacle_j.1 && player_j.1 > obstacle_j.0 && player.i == obstacle.i {
            scoreboard.score -= 1;
            player.i = PLAYER_INITIAL_POSITION.x;
            player.j = PLAYER_INITIAL_POSITION.z;
            player_transform.translation = PLAYER_INITIAL_POSITION;
        }
    }
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
            if player.i < BOARD_SIZE_I as f32 - 1.0 {
                player.i += 1.0;
            }
            rotation = PI / 2.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            if player.i > 0.0 {
                player.i -= 1.0;
            }
            rotation = -PI / 2.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            if player.j < BOARD_SIZE_J as f32 - 1.0 {
                player.j += 1.0;
            }
            rotation = 0.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            if player.j > 0.0 {
                player.j -= 1.0;
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

fn move_obstacle(mut obstacle_query: Query<(&Transform, &mut Velocity), With<Obstacle>>) {
    for (obstacle_transform, mut obstacle_velocity) in &mut obstacle_query {
        let left_board_collision =
            0.0 > obstacle_transform.translation.z;
        let right_board_collision = (BOARD_SIZE_J as f32)
            < obstacle_transform.translation.z + obstacle_transform.scale.z;

        if left_board_collision || right_board_collision {
            obstacle_velocity.z = -obstacle_velocity.z;
        }
    }
}

fn focus_camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut Camera, &mut Transform), (With<Camera3d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let (mut camera, mut camera_transform) = camera_query.single_mut();
    let player_transform = player_query.single();
    let motion_time = CAMERA_SPEED * time.delta_seconds();

    // move camera position
    let mut camera_motion =
        player_transform.translation + CAMERA_DISTANCE - camera_transform.translation;
    if camera_motion.length() > 0.2 {
        camera_motion *= motion_time;
        camera_transform.translation += camera_motion;
    }

    // move camera looking position
    let mut camera_motion = player_transform.translation - camera.looking_at;
    if camera_motion.length() > 0.2 {
        camera_motion *= motion_time;
        camera.looking_at += camera_motion;
    }
    *camera_transform = camera_transform.looking_at(camera.looking_at, Vec3::Y);
}

fn goal_player(
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();

    if player.i >= BOARD_SIZE_I as f32 - 1.0 {
        scoreboard.score += 1;
        player.i = PLAYER_INITIAL_POSITION.x;
        player.j = PLAYER_INITIAL_POSITION.z;
        player_transform.translation = PLAYER_INITIAL_POSITION;
    }
}
