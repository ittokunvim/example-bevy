use bevy::prelude::*;

const GAME_TITLE: &str = "2D Setup";
const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);
const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

const MAINMENU_FONT_SIZE: f32 = 40.0;
const MAINMENU_FONT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const MAINMENU_BG_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const MAINMENU_SIZE: Vec2 = Vec2::new(400.0, 400.0);
const MAINMENU_TEXT_PLAY: &str = "Play";
const MAINMENU_TEXT_QUIT: &str = "Quit";
const MAINMENU_GAP: f32 = 80.0;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                title: GAME_TITLE.to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // MainMunu Title
    commands.spawn(
        TextBundle::from_section(
            GAME_TITLE,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: MAINMENU_FONT_SIZE,
                color: MAINMENU_FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - MAINMENU_FONT_SIZE / 2.0 - MAINMENU_SIZE.y / 2.0 + MAINMENU_FONT_SIZE),
            justify_self: JustifySelf::Center,
            ..default()
        })
    );
    // MainMenu Play
    commands.spawn(
        TextBundle::from_section(
            MAINMENU_TEXT_PLAY,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: MAINMENU_FONT_SIZE,
                color: MAINMENU_FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - MAINMENU_FONT_SIZE),
            justify_self: JustifySelf::Center,
            ..default()
        })
    );
    // MainMenu Quit
    commands.spawn(
        TextBundle::from_section(
            MAINMENU_TEXT_QUIT,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: MAINMENU_FONT_SIZE,
                color: MAINMENU_FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - MAINMENU_FONT_SIZE + MAINMENU_GAP),
            justify_self: JustifySelf::Center,
            ..default()
        })
    );
    // MainMenu Background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: MAINMENU_BG_COLOR,
            custom_size: Some(MAINMENU_SIZE),
            ..default()
        },
        ..default()
    });
}
