use bevy::prelude::*;
use bevy::log::LogPlugin;

const GAMETITLE: &str = "App State";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,example_bevy=debug";

const KEY_MAINMENU_TO_INGAME: KeyCode = KeyCode::KeyI;
const KEY_INGAME_TO_PAUSE: KeyCode = KeyCode::KeyP;
const KEY_INGAME_TO_GAMEOVER: KeyCode = KeyCode::KeyG;
const KEY_PAUSE_TO_INGAME: KeyCode = KeyCode::KeyP;
const KEY_GAMEOVER_TO_MAINMENU: KeyCode = KeyCode::KeyB;
const KEY_GAMEOVER_TO_INGAME: KeyCode = KeyCode::KeyR;

#[derive(Component)]
struct Mainmenu;

#[derive(Component)]
struct Ingame;

#[derive(Component)]
struct Pause;

#[derive(Component)]
struct Gameover;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Resource)]
enum AppState {
    #[default]
    Mainmenu,
    Ingame,
    Pause,
    Gameover,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.into(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(LogPlugin {
                filter: LOG_FILTER.into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            })
         )
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        // メインメニュー
        .add_systems(OnEnter(AppState::Mainmenu), mainmenu_setup)
        .add_systems(Update, mainmenu_update.run_if(in_state(AppState::Mainmenu)))
        .add_systems(OnExit(AppState::Mainmenu), mainmenu_exit)
        // ゲーム
        .add_systems(OnEnter(AppState::Ingame), ingame_setup)
        .add_systems(Update, ingame_update.run_if(in_state(AppState::Ingame)))
        .add_systems(OnExit(AppState::Ingame), ingame_exit)
        // ポーズ
        .add_systems(OnEnter(AppState::Pause), pause_setup)
        .add_systems(Update, pause_update.run_if(in_state(AppState::Pause)))
        .add_systems(OnExit(AppState::Pause), pause_exit)
        // ゲームオーバー
        .add_systems(OnEnter(AppState::Gameover), gameover_setup)
        .add_systems(Update, gameover_update.run_if(in_state(AppState::Gameover)))
        .add_systems(OnExit(AppState::Gameover), gameover_exit)
        .run();
}

/// カメラをセットアップする関数
fn setup(mut commands: Commands) {
    info_once!("setup");

    commands.spawn(Camera2d::default());
}

/// メインメニューのセットアップを行う関数
fn mainmenu_setup(mut commands: Commands) {
    info_once!("mainmenu_setup");

    let text = format!("State transition to Ingame with {:?}", KEY_MAINMENU_TO_INGAME);
    commands.spawn((
        Text2d(text),
        Mainmenu,
    ));
}

/// 特定のキーが押された時にMainmenuステートからIngameステートに遷移する関数
fn mainmenu_update(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("mainmenu_update");

    if keyboard_input.just_pressed(KEY_MAINMENU_TO_INGAME) {
        next_state.set(AppState::Ingame);
    }
}

/// Mainmenuコンポーネントを全て削除する関数
fn mainmenu_exit(
    mut commands: Commands,
    query: Query<Entity, With<Mainmenu>>,
) {
    info_once!("mainmenu_exit");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// ゲームのセットアップを行う関数
fn ingame_setup(mut commands: Commands) {
    info_once!("ingame_setup");

    let text = format!(
        "State transition to Pause with {:?}\nState translation to Gameover with {:?}",
        KEY_INGAME_TO_PAUSE, KEY_INGAME_TO_GAMEOVER,
    );
    commands.spawn((
        Text2d(text),
        Ingame,
    ));
}

/// 特定のキーが押された時にキーに対応するステートに遷移する関数
fn ingame_update(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("ingame_update");

    if keyboard_input.just_pressed(KEY_INGAME_TO_PAUSE) {
        next_state.set(AppState::Pause);
    }

    if keyboard_input.just_pressed(KEY_INGAME_TO_GAMEOVER) {
        next_state.set(AppState::Gameover);
    }
}

/// Ingameコンポーネントを全て削除する関数
fn ingame_exit(
    mut commands: Commands,
    query: Query<Entity, With<Ingame>>,
) {
    info_once!("ingame_exit");
    
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// ポーズのセットアップを行う関数
fn pause_setup(mut commands: Commands) {
    info_once!("pause_setup");

    let text = format!("State transition to Ingame with {:?}", KEY_PAUSE_TO_INGAME);
    commands.spawn((
        Text2d(text),
        Pause,
    ));
}

/// 特定のキーが押された時にPauseステートからIngameステートに遷移する関数
fn pause_update(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("pause_update");

    if keyboard_input.just_pressed(KEY_PAUSE_TO_INGAME) {
        next_state.set(AppState::Ingame);
    }
}

/// Pauseコンポーネントを全て削除する関数
fn pause_exit(
    mut commands: Commands,
    query: Query<Entity, With<Pause>>,
) {
    info_once!("pause_exit");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// ゲームオーバーのセットアップを行う関数
fn gameover_setup(mut commands: Commands) {
    info_once!("gameover_setup");

    let text = format!(
        "State transition to Mainmenu with {:?}\nState transition to Ingame with {:?}",
        KEY_GAMEOVER_TO_MAINMENU, KEY_GAMEOVER_TO_INGAME,
    );
    commands.spawn((
        Text2d(text),
        Gameover,
    ));
}

/// 特定のキーが押された時にキーに対応するステートに遷移する関数
fn gameover_update(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("gameover_update");

    if keyboard_input.just_pressed(KEY_GAMEOVER_TO_MAINMENU) {
        next_state.set(AppState::Mainmenu);
    }

    if keyboard_input.just_pressed(KEY_GAMEOVER_TO_INGAME) {
        next_state.set(AppState::Ingame);
    }
}

/// Gameoverコンポーネントを全て削除する関数
fn gameover_exit(
    mut commands: Commands,
    query: Query<Entity, With<Gameover>>,
) {
    info_once!("gameover_exit");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}
