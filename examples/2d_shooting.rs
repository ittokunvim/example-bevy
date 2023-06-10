use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const WINDOW_SIZE: Vec2 = Vec2::new(700.0, 700.0);

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 30.0;
const GAP_BETWEEN_PLAYER_AND_FLOOR: f32 = 40.0;
const PLAYER_PADDING: f32 = 20.0;

const BULLET_SPEED: f32 = 800.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.9, 0.3);

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
        .add_system(apply_velocity)
        .add_system(move_player)
        .add_system(shot_player)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    let player_y = -WINDOW_SIZE.y / 2. + GAP_BETWEEN_PLAYER_AND_FLOOR;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(PLAYER_SIZE, 3).into())
                .into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            transform: Transform::from_translation(Vec3::new(0., player_y, 0.)),
            ..default()
        },
        Player,
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction = Vec2::ZERO;

    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    }

    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    }

    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.0;
    }

    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.0;
    }

    // Player x movement
    let new_player_position_x = player_transform.translation.x
        + direction.x * PLAYER_SPEED * time_step.period.as_secs_f32();
    let left_bound = -WINDOW_SIZE.x / 2.0 + PLAYER_SIZE / 2.0 + PLAYER_PADDING;
    let right_bound = WINDOW_SIZE.x / 2.0 - PLAYER_SIZE / 2.0 - PLAYER_PADDING;

    // Player y movement
    let new_player_position_y = player_transform.translation.y
        + direction.y * PLAYER_SPEED * time_step.period.as_secs_f32();
    let up_bound = -WINDOW_SIZE.y / 2.0 + PLAYER_SIZE / 2.0 + PLAYER_PADDING;
    let down_bound = WINDOW_SIZE.y / 2.0 - PLAYER_SIZE / 2.0 - PLAYER_PADDING;

    player_transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);
    player_transform.translation.y = new_player_position_y.clamp(up_bound, down_bound);
}

fn shot_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();

    if keyboard_input.just_pressed(KeyCode::Space) {
        // Spawn a bullet
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
                transform: Transform::from_translation(player_transform.translation),
                ..default()
            },
            Velocity(Vec2::new(0., 0.5) * BULLET_SPEED),
        ));
    }
}
