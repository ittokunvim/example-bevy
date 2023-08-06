use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::MaterialMesh2dBundle;

use rand::Rng;

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const GAME_TIME_LIMIT: f32 = 60.0;

const PRESSANYKEY_FONT_SIZE: f32 = 30.0;
const PRESSANYKEY_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const PRESSANYKEY_TEXT_PADDING: f32 = 20.0;

const PLAYER_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.1, 0.8, 0.1);
const PLAYER_SPEED: f32 = 200.0;

const OBSTACLE_SPAWN_INTERVAL: f32 = 0.5;
const OBSTACLE_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const OBSTACLE_GOOD_COLOR: Color = Color::rgb(0.1, 0.1, 0.8);
const OBSTACLE_BAD_COLOR: Color = Color::rgb(0.8, 0.1, 0.1);
const OBSTACLE_SPEED: f32 = 2.5;

const SCOREBOARD_FONT_SIZE: f32 = 30.0;
const SCOREBOARD_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const SCOREBOARD_TEXT_PADDING: f32 = 5.0;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Resource)]
struct ObstacleSpawnTimer(Timer);

#[derive(Resource, Component)]
struct Scoreboard {
    time: f32,
    score: i32,
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
        .insert_resource(GameTimer(Timer::from_seconds(
            GAME_TIME_LIMIT,
            TimerMode::Once,
        )))
        .insert_resource(ObstacleSpawnTimer(Timer::from_seconds(
            OBSTACLE_SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .insert_resource(Scoreboard {
            time: GAME_TIME_LIMIT,
            score: 0,
        })
        .add_systems(Startup, setup_camera)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(Update, move_player.run_if(in_state(AppState::InGame)))
        .add_systems(Update, spawn_obstacle.run_if(in_state(AppState::InGame)))
        .add_systems(Update, move_obstacle.run_if(in_state(AppState::InGame)))
        .add_systems(Update, collide_obstacle.run_if(in_state(AppState::InGame)))
        .add_systems(Update, cleanup_obstacle.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_game_timer.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_scoreboard.run_if(in_state(AppState::InGame)))
        .add_systems(OnExit(AppState::InGame), teardown)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::GameOver)))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct PressAnyKey;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle {
    point: i32,
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Player
    let player_y = -WINDOW_SIZE.y / 2.0 + PLAYER_SIZE.y;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(1.0, 4).into()).into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            transform: Transform {
                translation: Vec3::new(0.0, player_y, 1.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            ..default()
        },
        Player,
    ));

    // Scoreboard
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Time: ",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                },
            ),
            TextSection::new(
                GAME_TIME_LIMIT.to_string(),
                TextStyle {
                    font: font_medium.clone(),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                },
            ),
            TextSection::new(
                " | Score: ",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: font_medium.clone(),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(SCOREBOARD_TEXT_PADDING),
            left: Val::Px(SCOREBOARD_TEXT_PADDING),
            ..default()
        }),
        Scoreboard {
            time: GAME_TIME_LIMIT,
            score: 0,
        },
    ));
}

fn press_any_key(
    asset_server: Res<AssetServer>,
    mut keyboard_event: EventReader<KeyboardInput>,
    pressanykey_query: Query<Entity, With<PressAnyKey>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut inkey: ResMut<Input<KeyCode>>,
) {
    if pressanykey_query.is_empty() {
        // Press any key
        let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");

        commands.spawn((
            TextBundle::from_section(
                "Press Any Key ...",
                TextStyle {
                    font: font_bold,
                    font_size: PRESSANYKEY_FONT_SIZE,
                    color: PRESSANYKEY_COLOR,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(PRESSANYKEY_TEXT_PADDING),
                right: Val::Px(PRESSANYKEY_TEXT_PADDING),
                ..default()
            }),
            PressAnyKey,
        ));
    }

    for _event in keyboard_event.iter() {
        if let Ok(pressanykey_entity) = pressanykey_query.get_single() {
            commands.entity(pressanykey_entity).despawn();

            app_state.set(AppState::InGame);
            inkey.reset_all();
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction = Vec2::ZERO;

    // Keyboard input
    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    }

    // Player x movement
    let new_player_position_x = player_transform.translation.x
        + direction.x * PLAYER_SPEED * time_step.period.as_secs_f32();
    let x_bound = WINDOW_SIZE.x / 2.0 - PLAYER_SIZE.x;

    player_transform.translation.x = new_player_position_x.clamp(-x_bound, x_bound);
}

fn spawn_obstacle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<ObstacleSpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // Obstacle
        let x_bound = WINDOW_SIZE.x / 2.0 - OBSTACLE_SIZE.x;
        let obstacle_x = rand::thread_rng().gen_range(-x_bound..x_bound);
        let obstacle_y = WINDOW_SIZE.y / 2.0 + OBSTACLE_SIZE.y;
        let bool_obstacle = rand::thread_rng().gen_bool(1.0 / 2.0);
        let obstacle_color = if bool_obstacle { OBSTACLE_GOOD_COLOR } else { OBSTACLE_BAD_COLOR };
        let obstacle_point = if bool_obstacle { 1 } else { -1 };

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(1.0).into()).into(),
                material: materials.add(ColorMaterial::from(obstacle_color)),
                transform: Transform {
                    translation: Vec3::new(obstacle_x, obstacle_y, 0.0),
                    scale: OBSTACLE_SIZE,
                    ..default()
                },
                ..default()
            },
            Obstacle { point: obstacle_point },
        ));
    }
}

fn move_obstacle(mut obstacle_query: Query<&mut Transform, With<Obstacle>>) {
    for mut obstacle_transform in obstacle_query.iter_mut() {
        obstacle_transform.translation.y -= OBSTACLE_SPEED;
    }
}

fn collide_obstacle(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    obstacle_query: Query<(&Obstacle, Entity, &Transform), With<Obstacle>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let player = player_query.single();
    let player_size = player.scale.truncate();

    for (obstacle, obstacle_entity, obstacle_transform) in obstacle_query.iter() {
        let obstacle_size = obstacle_transform.scale.truncate();
        let collision = collide(
            player.translation,
            player_size,
            obstacle_transform.translation,
            obstacle_size,
        );

        if let Some(..) = collision {
            scoreboard.score += obstacle.point;
            commands.entity(obstacle_entity).despawn();
        }
    }
}

fn cleanup_obstacle(
    mut commands: Commands,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    for (obstacle_entity, obstacle_transform) in obstacle_query.iter() {
        let obstacle_pos = obstacle_transform.translation;
        let window_half_size = WINDOW_SIZE / 2.0 + OBSTACLE_SIZE.truncate();

        if obstacle_pos.x < -window_half_size.x
            || obstacle_pos.x > window_half_size.x
            || obstacle_pos.y < -window_half_size.y
            || obstacle_pos.y > window_half_size.y
        {
            commands.entity(obstacle_entity).despawn();
        }
    }
}

fn update_game_timer(
    mut scoreboard: ResMut<Scoreboard>,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    scoreboard.time = timer.0.remaining_secs().round();

    if timer.0.tick(time.delta()).just_finished() {
        timer.0.reset();
        app_state.set(AppState::GameOver);
    }
}

fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut scoreboard_query: Query<&mut Text, With<Scoreboard>>,
) {
    let mut scoreboard_text = scoreboard_query.single_mut();
    scoreboard_text.sections[1].value = scoreboard.time.to_string();
    scoreboard_text.sections[3].value = scoreboard.score.to_string();
}

fn teardown(
    mut commands: Commands,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    scoreboard.score = 0;

    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
