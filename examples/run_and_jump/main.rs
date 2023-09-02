use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::sprite::MaterialMesh2dBundle;

use serde::{Deserialize, Serialize};

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const TILE_SIZE: f32 = 40.0;
const TILE_GROUND_COLOR: Color = Color::rgb(0.5, 0.3, 0.2);
const TILE_GOAL_COLOR: Color = Color::rgb(0.8, 0.8, 0.2);

const PLAYER_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.1, 0.8, 0.1);
const PLAYER_SPEED: f32 = 100.0;
const PLAYER_GRAVITY: f32 = 3.0;
const PLAYER_JUMP: f32 = 30.0;
const PLAYER_JUMP_COUNT: u32 = 2;

const CAMERA_FOCUS_OFFSET: f32 = -200.0;

const PRESSANYKEY_FONT_SIZE: f32 = 30.0;
const PRESSANYKEY_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const PRESSANYKEY_TEXT_PADDING: f32 = 20.0;

const RESULT_TEXT_PADDING: f32 = 40.0;
const RESULT_BACKGROUND_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const RESULT_FONT_SIZE: f32 = 50.0;
const GAMEOVER_FONT_COLOR: Color = Color::rgb(0.9, 0.1, 0.1);
const GAMECLEAR_FONT_COLOR: Color = Color::rgb(0.1, 0.9, 0.1);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameClear,
    GameOver,
}

#[derive(Resource)]
struct StageCount(u32);

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
        .insert_resource(StageCount(1))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))

        .add_systems(OnEnter(AppState::InGame), setup_tilemap)
        .add_systems(OnEnter(AppState::InGame), setup_player)
        .add_systems(Update, apply_velocity.run_if(in_state(AppState::InGame)))
        .add_systems(Update, focus_camera_on_player.run_if(in_state(AppState::InGame)))
        .add_systems(Update, player_gravity.run_if(in_state(AppState::InGame)))
        .add_systems(Update, ground_collision.run_if(in_state(AppState::InGame)))
        .add_systems(Update, goal_collision.run_if(in_state(AppState::InGame)))
        .add_systems(Update, jump_player.run_if(in_state(AppState::InGame)))

        .add_systems(OnEnter(AppState::GameClear), display_gameclear)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::GameClear)))
        .add_systems(OnExit(AppState::GameClear), teardown)

        .add_systems(OnEnter(AppState::GameOver), display_gameover)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::GameOver)))
        .add_systems(OnExit(AppState::GameOver), teardown)

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
struct TileGoal;

#[derive(Component)]
struct Player {
    vel_y: f32,
    jump_count: u32,
    on_ground: bool,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Component)]
struct PressAnyKey;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_tilemap(stage_count: u32) -> TileMap {
    match stage_count {
        1 => serde_json::from_slice(include_bytes!("stage_1.json")).unwrap(),
        2 => serde_json::from_slice(include_bytes!("stage_2.json")).unwrap(),
        3 => serde_json::from_slice(include_bytes!("stage_3.json")).unwrap(),
        4 => serde_json::from_slice(include_bytes!("stage_4.json")).unwrap(),
        5 => serde_json::from_slice(include_bytes!("stage_5.json")).unwrap(),
        _ => panic!("invalid stage count"),
    }
}

fn setup_tilemap(mut commands: Commands, stage_count: Res<StageCount>) {
    let stage_count = stage_count.0;
    let tile_map: TileMap = load_tilemap(stage_count);
    let window_top_left = Vec2::new(-WINDOW_SIZE.x / 2.0, WINDOW_SIZE.y / 2.0);

    for (y, row) in tile_map.map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell > &0 {
                let tile_y = window_top_left.y - TILE_SIZE * (y as f32 + 0.5) as f32;
                let tile_x = window_top_left.x + TILE_SIZE * (x as f32 + 0.5) as f32;

                let tile_ground = (
                    SpriteBundle {
                        sprite: Sprite {
                            color: TILE_GROUND_COLOR,
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
                );
                let tile_goal = (
                    SpriteBundle {
                        sprite: Sprite {
                            color: TILE_GOAL_COLOR,
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(tile_x, tile_y, 0.0),
                            scale: Vec3::splat(TILE_SIZE),
                            ..default()
                        },
                        ..default()
                    },
                    TileGoal,
                );

                match cell {
                    1 => commands.spawn(tile_ground),
                    2 => commands.spawn(tile_goal),
                    _ => commands.spawn(SpriteBundle::default()),
                };
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
        Player {
            vel_y: 0.0,
            jump_count: PLAYER_JUMP_COUNT,
            on_ground: false,
        },
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

fn player_gravity(
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        if !player.on_ground {
            player_transform.translation.y -= PLAYER_GRAVITY;
            if player.jump_count >= PLAYER_JUMP_COUNT {
                player.jump_count = 1;
            }
        }

        if player_transform.translation.y < -WINDOW_SIZE.y / 2.0 - player_transform.scale.y {
            app_state.set(AppState::GameOver);
        }
    }
}

fn ground_collision(
    mut player_query: Query<(&Transform, &mut Player, &mut Velocity), With<Player>>,
    ground_query: Query<&Transform, (With<TileGround>, Without<Player>)>,
) {
    let (player_transform, mut player, mut player_velocity) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();
    player.on_ground = false;
    player_velocity.x = PLAYER_SPEED;

    for ground_transform in ground_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_size,
            ground_transform.translation,
            ground_transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            match collision {
                Collision::Top => {
                    player.on_ground = true;
                    player.jump_count = PLAYER_JUMP_COUNT;
                }
                _ => {}
            }
        }
    }
}

fn goal_collision(
    player_query: Query<&Transform, With<Player>>,
    goal_query: Query<&Transform, (With<TileGoal>, Without<Player>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let player_transform = player_query.single();
    let player_size = player_transform.scale.truncate();

    for goal_transform in goal_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_size,
            goal_transform.translation,
            goal_transform.scale.truncate(),
        );

        if let Some(_) = collision {
            app_state.set(AppState::GameClear);
        }
    }
}

fn jump_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
) {
    if let Ok((mut player, mut player_transform)) = player_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.jump_count > 0 {
                player.vel_y += PLAYER_JUMP;
                player.jump_count -= 1;
            }
        }

        if player.vel_y > 0.0 {
            player.vel_y -= PLAYER_GRAVITY;
            player_transform.translation.y += player.vel_y;
        }
    }
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

fn teardown(
    mut commands: Commands,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;

    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn display_gameover(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text_parent = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };
    let text_background = NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(RESULT_TEXT_PADDING)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: RESULT_BACKGROUND_COLOR.into(),
        ..default()
    };

    let text = TextBundle::from_sections([TextSection::new(
        "Game Over",
        TextStyle {
            font: font_bold.clone(),
            font_size: RESULT_FONT_SIZE,
            color: GAMEOVER_FONT_COLOR,
        },
    )]);

    commands.spawn(text_parent).with_children(|parent| {
        parent.spawn(text_background).with_children(|parent| {
            parent.spawn(text);
        });
    });
}

fn display_gameclear(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text_parent = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };
    let text_background = NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(RESULT_TEXT_PADDING)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: RESULT_BACKGROUND_COLOR.into(),
        ..default()
    };

    let text = TextBundle::from_sections([TextSection::new(
        "Game Clear",
        TextStyle {
            font: font_bold.clone(),
            font_size: RESULT_FONT_SIZE,
            color: GAMECLEAR_FONT_COLOR,
        },
    )]);

    commands.spawn(text_parent).with_children(|parent| {
        parent.spawn(text_background).with_children(|parent| {
            parent.spawn(text);
        });
    });
}
