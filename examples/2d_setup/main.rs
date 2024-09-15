use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::mainmenu::{
    mainmenu_setup,
    mainmenu_update,
};
use crate::ingame::{
    PlayerBundle,
    WallBundle,
    GoalBundle,
    LevelWalls,
    ingame_setup,
    move_player_from_input,
    translate_grid_coords_entities,
    cache_wall_locations,
    check_goal,
    check_pause,
    check_ldtk_transform,
};
use crate::pause::{
    pause_setup,
    pause_update,
};
use crate::gameover::{
    gameover_setup,
    gameover_update,
};

pub mod mainmenu;
pub mod ingame;
pub mod pause;
pub mod gameover;

pub const GAME_TITLE: &str = "2D Setup";
pub const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);
pub const BACKGROUND_COLOR: Color = Color::rgb(0.255, 0.251, 0.333);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Pause,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAME_TITLE.to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        // ldtk setup
        .add_plugins(LdtkPlugin)
        .init_resource::<LevelWalls>()
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_int_cell::<WallBundle>(1)
        .add_systems(Startup, setup_camera)
        // mainmenu
        .add_systems(OnEnter(AppState::MainMenu), mainmenu_setup)
        .add_systems(Update, mainmenu_update.run_if(in_state(AppState::MainMenu)))
        // ingame
        .add_systems(OnEnter(AppState::InGame), ingame_setup)
        .add_systems(Update, (
            move_player_from_input,
            translate_grid_coords_entities,
            cache_wall_locations,
            check_goal,
            check_pause,
            check_ldtk_transform,
        ).run_if(in_state(AppState::InGame)))
        // pause
        .add_systems(OnEnter(AppState::Pause), pause_setup)
        .add_systems(Update, pause_update.run_if(in_state(AppState::Pause)))
        // gameover
        .add_systems(OnEnter(AppState::GameOver), gameover_setup)
        .add_systems(Update, gameover_update.run_if(in_state(AppState::GameOver)))
        .run();
}   

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
