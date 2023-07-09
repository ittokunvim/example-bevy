use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::MaterialMesh2dBundle;

const WINDOW_SIZE: Vec2 = Vec2::new(700.0, 700.0);

const PLAYER_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const PLAYER_JUMP: f32 = 25.0;
const PLAYER_GRAVITY: f32 = 2.0;
const PLAYER_COLLIDE_COOLDOWN: f32 = 1.0;

const OBSTACLE_SIZE: Vec3 = Vec3::new(50.0, WINDOW_SIZE.y / 2.0 - OBSTACLE_SPACE / 2.0, 0.0);
const OBSTACLE_SPACE: f32 = 200.0;
const OBSTACLE_SPEED: f32 = 200.0;

const SCOREBOARD_FONT_SIZE: f32 = 24.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PLAYER_COLOR: Color = Color::rgb(0.1, 0.8, 0.1);
const OBSTACLE_COLOR: Color = Color::rgb(0.8, 0.1, 0.1);
const TEXT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const SCOREBOARD_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Resource)]
struct ObstacleSpawnTimer(Timer);

#[derive(Resource)]
struct Scoreboard {
    score: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ObstacleSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .insert_resource(Scoreboard { score: -1.0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_startup_system(setup)
        .add_system(apply_velocity)
        .add_system(jump_player)
        .add_system(player_gravity)
        .add_system(spawn_obstacles)
        .add_system(despawn_obstacles)
        .add_system(obstacle_collision)
        .add_system(pass_obstacle)
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player {
    vel_y: f32,
    collide_cooldown: Timer,
}

#[derive(Component)]
struct Obstacle {
    is_passed: bool,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(1.0, 4).into()).into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            ..default()
        },
        Player {
            vel_y: 0.0,
            collide_cooldown: Timer::from_seconds(PLAYER_COLLIDE_COOLDOWN, TimerMode::Once),
        },
    ));

    // Scoreboard
    let font_bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: font_bold,
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_medium,
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCOREBOARD_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
}

fn jump_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) {
        player.vel_y += PLAYER_JUMP;
    }

    if player.vel_y > 0.0 {
        player.vel_y -= PLAYER_GRAVITY;
        player_transform.translation.y += player.vel_y;
    }
}

fn player_gravity(mut player_query: Query<&mut Transform, With<Player>>) {
    let mut player_transform = player_query.single_mut();

    player_transform.translation.y -= PLAYER_GRAVITY;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
    }
}

fn spawn_obstacles(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ObstacleSpawnTimer>) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    let x = WINDOW_SIZE.x / 2.0 + OBSTACLE_SIZE.x / 2.0;
    let mut y = WINDOW_SIZE.y / 2.0 - OBSTACLE_SIZE.y / 2.0;

    for i in 0..2 {
        if i == 1 {
            y = -y;
        }

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: OBSTACLE_COLOR,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec2::new(x, y).extend(0.0),
                    scale: OBSTACLE_SIZE,
                    ..default()
                },
                ..default()
            },
            Obstacle { is_passed: false },
            Velocity(Vec3::new(-OBSTACLE_SPEED, 0., 0.)),
        ));
    }
}

fn despawn_obstacles(
    mut commands: Commands,
    mut obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    for (obstacle_entity, obstacle_transform) in &mut obstacle_query {
        if obstacle_transform.translation.x < -WINDOW_SIZE.x / 2.0 - OBSTACLE_SIZE.x / 2.0 {
            commands.entity(obstacle_entity).despawn();
        }
    }
}

fn obstacle_collision(
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    time: Res<Time>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();

    if !player.collide_cooldown.tick(time.delta()).finished() {
        return;
    }

    for obstacle_transform in &obstacle_query {
        let collision = collide(
            player_transform.translation,
            player_size,
            obstacle_transform.translation,
            obstacle_transform.scale.truncate(),
        );

        if let Some(_collision) = collision {
            player.collide_cooldown.reset();
        }
    }
}

fn pass_obstacle(
    player_query: Query<&Transform, With<Player>>,
    mut obstacle_query: Query<(&mut Obstacle, &Transform), With<Obstacle>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let player_transform = player_query.single();

    for (mut obstacle, obstacle_transform) in &mut obstacle_query {
        if obstacle.is_passed {
            continue;
        }

        if player_transform.translation.x < obstacle_transform.translation.x {
            obstacle.is_passed = true;
            scoreboard.score += 0.5;
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
