use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::MaterialMesh2dBundle;

const WINDOW_SIZE: Vec2 = Vec2::new(700.0, 700.0);

const PLAYER_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const PLAYER_JUMP: f32 = 35.0;
const PLAYER_GRAVITY: f32 = 3.0;
const PLAYER_COLLIDE_COOLDOWN: f32 = 1.0;
const PLAYER_LIFE: usize = 3;

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
    life: usize,
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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .insert_resource(ObstacleSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .insert_resource(Scoreboard {
            score: -1.0,
            life: PLAYER_LIFE,
        })
        .add_startup_system(setup)
        .add_systems((jump_player, player_gravity, despawn_player))
        .add_systems((
            spawn_obstacles,
            despawn_obstacles,
            obstacle_collision,
            pass_obstacle,
        ))
        .add_systems((apply_velocity, update_scoreboard))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player {
    vel_y: f32,
    collide_cooldown: Timer,
    life: usize,
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
            life: PLAYER_LIFE,
        },
    ));

    // Scoreboard
    let font_bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_closure = |font: Handle<Font>, text: &str, color: Color| -> TextSection {
        let style = TextStyle {
            font,
            font_size: SCOREBOARD_FONT_SIZE,
            color,
        };
        TextSection::new(text, style)
    };

    commands.spawn(
        TextBundle::from_sections([
            text_closure(font_bold.clone(), "Score: ", TEXT_COLOR),
            text_closure(font_medium.clone(), "", SCOREBOARD_COLOR),
            text_closure(font_bold.clone(), ", Life: ", TEXT_COLOR),
            text_closure(font_medium.clone(), "", SCOREBOARD_COLOR),
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
    if let Ok((mut player, mut player_transform)) = player_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            player.vel_y += PLAYER_JUMP;
        }

        if player.vel_y > 0.0 {
            player.vel_y -= PLAYER_GRAVITY;
            player_transform.translation.y += player.vel_y;
        }
    }
}

fn player_gravity(mut player_query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        player_transform.translation.y -= PLAYER_GRAVITY;
    }
}

fn despawn_player(mut commands: Commands, player_query: Query<(Entity, &Player), With<Player>>) {
    if let Ok((player_entity, player)) = &player_query.get_single() {
        if player.life == 0 {
            commands.entity(*player_entity).despawn();
        }
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
    mut scoreboard: ResMut<Scoreboard>,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        if !player.collide_cooldown.tick(time.delta()).finished() {
            return;
        }

        let player_size = player_transform.scale.truncate();

        for obstacle_transform in &obstacle_query {
            let collision = collide(
                player_transform.translation,
                player_size,
                obstacle_transform.translation,
                obstacle_transform.scale.truncate(),
            );

            if let Some(_collision) = collision {
                player.collide_cooldown.reset();
                player.life -= 1;
                scoreboard.life -= 1;
            }
        }
    }
}

fn pass_obstacle(
    player_query: Query<&Transform, With<Player>>,
    mut obstacle_query: Query<(&mut Obstacle, &Transform), With<Obstacle>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    if let Ok(player_transform) = player_query.get_single() {
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
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
    text.sections[3].value = scoreboard.life.to_string();
}
