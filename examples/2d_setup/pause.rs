use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
};

const FONT_SIZE: f32 = 40.0;
const FONT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const BG_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BG_SIZE: Vec2 = Vec2::new(80.0, 80.0);
const PAUSEBTN_TEXT: &str = "Pause";

#[derive(Component)]
pub struct Pause;

pub fn pause_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Pause
    commands.spawn((
        TextBundle::from_section(
            PAUSEBTN_TEXT,
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
        Pause,
    ));
    // Pause Background
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
        Pause,
    ));
}

pub fn pause_update(
    keyboard_input: Res<Input<KeyCode>>,
    pause_query: Query<Entity, With<Pause>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        // Changed app state
        app_state.set(AppState::InGame);
        // Despawned pause entities
        for pause_entity in pause_query.iter() {
            commands.entity(pause_entity).despawn();
        }
    }
}
