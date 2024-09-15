use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    AppState,
    WINDOW_SIZE,
};

const FONT_SIZE: f32 = 40.0;
const FONT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BG_COLOR: Color = Color::rgb(0.317, 0.337, 0.411);
const BG_SIZE: Vec2 = Vec2::new(140.0, 180.0);
const TEXT_GAP: f32 = 80.0;
const GAMEOVER_TEXT: &str = "Game Over";
const RESTART_TEXT: &str = "Restart";
const BACKTITLE_TEXT: &str = "Back to Title";

#[derive(Component)]
pub struct Gameover;

#[derive(Component)]
pub struct Restart;

#[derive(Component)]
pub struct BackTitle;

pub fn gameover_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Gameover
    commands.spawn((
        TextBundle::from_section(
            GAMEOVER_TEXT,
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
        Gameover,
    ));
    // Restart 
    commands.spawn((
        TextBundle::from_section(
            RESTART_TEXT,
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
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
        Gameover,
        Restart,
    ));
    // Back to Title
    commands.spawn((
        TextBundle::from_section(
            BACKTITLE_TEXT,
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
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
        Gameover,
        BackTitle,
    ));
    // Gameover Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BG_COLOR,
                custom_size: Some(BG_SIZE),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(
                    WINDOW_SIZE.x / 4.0,
                    WINDOW_SIZE.y / 4.0,
                    10.0),
                ..default()
            },
            ..default()
        },
        Gameover,
    ));
}

pub fn gameover_update(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_event: Res<Input<MouseButton>>,
    gameover_query: Query<Entity, With<Gameover>>,
    restart_query: Query<&Transform, With<Restart>>,
    backtitle_query: Query<&Transform, With<BackTitle>>,
    level_selection: ResMut<LevelSelection>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut ldtk_projects: Query<&mut Transform, (With<Handle<LdtkProject>>, Without<Restart>, Without<BackTitle>)>,
) {
    let window = window_query.single();
    let restart_transform = restart_query.single();
    let backtitle_transform = backtitle_query.single();

    if mouse_event.just_pressed(MouseButton::Left) {
        let cursor_position = window.cursor_position().unwrap();
        let restart_pos = restart_transform.translation.truncate();
        let backtitle_pos = backtitle_transform.translation.truncate();
        let restart_distance = cursor_position.distance(restart_pos);
        let backtitle_distance = cursor_position.distance(backtitle_pos);

        if restart_distance < 40.0 {
            // Reset level selection
            let indices = match level_selection.into_inner() {
                LevelSelection::Indices(indices) => indices,
                _ => panic!("level selection should always be Indices in this game"),
            };
            indices.level = 0;
            // Change game state
            app_state.set(AppState::InGame);
            // Despawned gameover Entities
            for gameover_entity in gameover_query.iter() {
                commands.entity(gameover_entity).despawn();
            }
        }
        else if backtitle_distance < 40.0 {
            // Change game state
            app_state.set(AppState::MainMenu);
            // Change ldtk transform
            let mut ldtk_projects_transform = ldtk_projects.single_mut();
            ldtk_projects_transform.translation.x = -99999.0;
            // Despawned gameover Entities
            for gameover_entity in gameover_query.iter() {
                commands.entity(gameover_entity).despawn();
            }
        }
    }
}
