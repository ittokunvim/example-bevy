use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

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
    InGame,
    // Pause,
    // GameOver,
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
        .add_systems(Startup, mainmenu_setup.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, mainmenu_update.run_if(in_state(AppState::MainMenu)))
        .add_systems(Startup, ingame_setup.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, ingame_update.run_if(in_state(AppState::InGame)))
        // .add_systems(Update, gameover.run_if(in_state(AppState::GameOver)))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}   

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

fn mainmenu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // MainMunu Title
    commands.spawn((
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
        }),
        MainMenu,
    ));
    // MainMenu Play
    commands.spawn((
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
        }),
        MainMenu,
        PlayButton,
    ));
    // MainMenu Quit
    commands.spawn((
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
        }),
        MainMenu,
        QuitButton,
    ));
    // MainMenu Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: MAINMENU_BG_COLOR,
                custom_size: Some(MAINMENU_SIZE),
                ..default()
            },
            ..default()
        },
        MainMenu,
    ));
}

fn mainmenu_update(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_event: Res<Input<MouseButton>>,
    mainmenu_query: Query<Entity, With<MainMenu>>,
    playbtn_query: Query<&Transform, With<PlayButton>>,
    quitbtn_query: Query<&Transform, With<QuitButton>>,
    mut commands: Commands,
    mut now_state: ResMut<State<AppState>>,
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
            for mainmenu_entity in mainmenu_query.iter() {
                commands.entity(mainmenu_entity).despawn();
            }
            *now_state = State::new(AppState::InGame);
        }

        if quitbtn_distance < 40.0 {
            exit.send(AppExit);
        }
    }
}

fn ingame_setup() {}

fn ingame_update() {}

// fn gameover() {}

