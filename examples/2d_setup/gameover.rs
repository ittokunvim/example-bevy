use bevy::prelude::*;

use crate::WINDOW_SIZE;

const FONT_SIZE: f32 = 40.0;
const FONT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BG_COLOR: Color = Color::rgb(0.317, 0.337, 0.411);
const GAMEOVER_SIZE: Vec2 = Vec2::new(140.0, 180.0);
const GAMEOVER_TEXT: &str = "Game Over";
const GAMEOVER_GAP: f32 = 80.0;
const RESTART_TEXT: &str = "Restart";
const BACKTITLE_TEXT: &str = "Back to Title";

#[derive(Component)]
pub struct Gameover;

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
            top: Val::Px(WINDOW_SIZE.y / 2.0 - FONT_SIZE / 2.0 - GAMEOVER_GAP),
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
            top: Val::Px(WINDOW_SIZE.y / 2.0 - FONT_SIZE / 2.0 + GAMEOVER_GAP),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Gameover,
    ));
    // Gameover Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BG_COLOR,
                custom_size: Some(GAMEOVER_SIZE),
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

pub fn gameover_update() {
    println!("running gameover_update");
}
