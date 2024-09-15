use bevy::{
    app::AppExit,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    GAME_TITLE,
    WINDOW_SIZE,
    AppState,
};

const FONT_SIZE: f32 = 40.0;
const FONT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const BG_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const SCREEN_SIZE: Vec2 = Vec2::new(400.0, 400.0);
const PLAYBTN_TEXT: &str = "Play";
const QUITBTN_TEXT: &str = "Quit";
const TEXT_GAP: f32 = 100.0;

#[derive(Component)]
pub struct Mainmenu;

#[derive(Component)]
pub struct PlayBtn;

#[derive(Component)]
pub struct QuitBtn;

pub fn mainmenu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,
) {
    // Camera
    let (mut camera_projection, mut camera_transform) = camera_query.single_mut();

    camera_projection.scale = 1.0;
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;

    // MainMunu Title
    commands.spawn((
        TextBundle::from_section(
            GAME_TITLE,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - FONT_SIZE / 2.0 - TEXT_GAP),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Mainmenu,
    ));
    // MainMenu Play
    commands.spawn((
        TextBundle::from_section(
            PLAYBTN_TEXT,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - FONT_SIZE / 2.0),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Mainmenu,
        PlayBtn,
    ));
    // MainMenu Quit
    commands.spawn((
        TextBundle::from_section(
            QUITBTN_TEXT,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - FONT_SIZE / 2.0 + TEXT_GAP),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Mainmenu,
        QuitBtn,
    ));
    // MainMenu Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BG_COLOR,
                custom_size: Some(SCREEN_SIZE),
                ..default()
            },
            ..default()
        },
        Mainmenu,
    ));
}

pub fn mainmenu_update(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_event: Res<Input<MouseButton>>,
    mainmenu_query: Query<Entity, With<Mainmenu>>,
    playbtn_query: Query<&Transform, With<PlayBtn>>,
    quitbtn_query: Query<&Transform, With<QuitBtn>>,
    level_selection: ResMut<LevelSelection>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    let window = window_query.single();
    let playbtn_transform = playbtn_query.single();
    let quitbtn_transform = quitbtn_query.single();

    if mouse_event.just_pressed(MouseButton::Left) {
        let cursor_position = window.cursor_position().unwrap();
        let playbtn_pos = playbtn_transform.translation.truncate();
        let quitbtn_pos = quitbtn_transform.translation.truncate();
        let playbtn_distance = cursor_position.distance(playbtn_pos);
        let quitbtn_distance = cursor_position.distance(quitbtn_pos);

        if playbtn_distance < 40.0 {
            // Reset game level
            let indices = match level_selection.into_inner() {
                LevelSelection::Indices(indices) => indices,
                _ => panic!("level selection should always be Indices in this game"),
            };
            indices.level = 0;
            // Changed app state
            app_state.set(AppState::InGame);
            // despawned Mainmenu entities
            for mainmenu_entity in mainmenu_query.iter() {
                commands.entity(mainmenu_entity).despawn();
            }
        }
        else if quitbtn_distance < 40.0 {
            // quit game
            exit.send(AppExit);
        }
    }
}
