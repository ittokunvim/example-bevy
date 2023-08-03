use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use rand::Rng;

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const PRESSANYKEY_FONT_SIZE: f32 = 30.0;
const PRESSANYKEY_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const PRESSANYKEY_TEXT_PADDING: f32 = 20.0;

const PLAYER_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.1, 0.8, 0.1);
const PLAYER_SPEED: f32 = 200.0;

const OBSTACLE_SPAWN_INTERVAL: f32 = 0.5;
const OBSTACLE_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const OBSTACLE_COLOR: Color = Color::rgb(0.8, 0.1, 0.1);
const OBSTACLE_SPEED: f32 = 2.5;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Resource)]
struct ObstacleSpawnTimer(Timer);

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
        .insert_resource(ObstacleSpawnTimer(Timer::from_seconds(
            OBSTACLE_SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, move_player.run_if(in_state(AppState::InGame)))
        .add_systems(Update, spawn_obstacle.run_if(in_state(AppState::InGame)))
        .add_systems(Update, move_obstacle.run_if(in_state(AppState::InGame)))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct PressAnyKey;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Press any key
    commands.spawn((
        TextBundle::from_section(
            "Press Any Key ...",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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
}

fn press_any_key(
    mut keyboard_event: EventReader<KeyboardInput>,
    pressanykey_query: Query<Entity, With<PressAnyKey>>,
    mut commands: Commands,
    mut now_state: ResMut<State<AppState>>,
    mut inkey: ResMut<Input<KeyCode>>,
) {
    for _event in keyboard_event.iter() {
        let pressanykey_entity = pressanykey_query.single();
        commands.entity(pressanykey_entity).despawn();

        *now_state = State::new(AppState::InGame);
        inkey.reset_all();
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

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(1.0).into()).into(),
                material: materials.add(ColorMaterial::from(OBSTACLE_COLOR)),
                transform: Transform {
                    translation: Vec3::new(obstacle_x, obstacle_y, 0.0),
                    scale: OBSTACLE_SIZE,
                    ..default()
                },
                ..default()
            },
            Obstacle,
        ));
    }
}

fn move_obstacle(mut obstacle_query: Query<&mut Transform, With<Obstacle>>) {
    for mut obstacle_transform in obstacle_query.iter_mut() {
        obstacle_transform.translation.y -= OBSTACLE_SPEED;
    }
}
