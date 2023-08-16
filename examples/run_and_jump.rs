use bevy::prelude::*;

use bevy_ecs_ldtk::prelude::*;

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WINDOW_SIZE.into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LdtkPlugin)
        .add_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..default()
        })
        .register_ldtk_entity::<LdtkBundle>("LdtkEntityIdentifier")
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_ldtk)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Bundle, LdtkEntity)]
struct LdtkBundle {
    #[sprite_sheet_bundle]
    sprite_sheet: SpriteSheetBundle,
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/run_and_jump.ldtk"),
        ..default()
    });
}
