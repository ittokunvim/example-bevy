use bevy::prelude::*;

use crate::mainmenu::mainmenu_setup;
use crate::mainmenu::mainmenu_update;

pub mod mainmenu;

pub const GAME_TITLE: &str = "2D Setup";
pub const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);
pub const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum AppState {
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

fn ingame_setup() {}

fn ingame_update() {}

// fn gameover() {}

