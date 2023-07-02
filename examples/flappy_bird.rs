use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const WINDOW_SIZE: Vec2 = Vec2::new(700.0, 700.0);

const PLAYER_SIZE: f32 = 25.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PLAYER_COLOR: Color = Color::rgb(0.1, 0.8, 0.1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(PLAYER_SIZE, 4).into())
                .into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
    ));
}
