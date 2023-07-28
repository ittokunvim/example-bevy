use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};

const WINDOW_SIZE: Vec2 = Vec2::new(700.0, 700.0);
const WINDOW_HALF_SIZE: Vec2 = Vec2::new(WINDOW_SIZE.x / 2.0, WINDOW_SIZE.y / 2.0);

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 15.0;
const PLAYER_HP: f32 = 3.0;
const GAP_BETWEEN_PLAYER_AND_FLOOR: f32 = 40.0;
const PLAYER_PADDING: f32 = 20.0;

const ENEMY_SPEED: f32 = 100.0;
const ENEMY_SIZE: f32 = 15.0;
const ENEMY_HP: f32 = 3.0;
const GAP_BETWEEN_ENEMY_AND_TOP: f32 = 40.0;
const INITIAL_ENEMY_DIRECTION: Vec2 = Vec2::new(-0.5, 0.0);
const ENEMY_ATTACK_INTERVAL: f32 = 0.2;

const SCOREBOARD_FONT_SIZE: f32 = 20.0;
const SCOREBOARD_TEXT_PADDING: f32 = 5.0;
const SCOREBOARD_SIZE: Vec2 = Vec2::new(
    WINDOW_SIZE.x,
    SCOREBOARD_FONT_SIZE + SCOREBOARD_TEXT_PADDING,
);

const BULLET_SPEED: f32 = 800.0;
const BULLET_SIZE: f32 = 5.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.9, 0.3);
const ENEMY_COLOR: Color = Color::rgb(0.9, 0.3, 0.3);
const TEXT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const SCOREBOARD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

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
        .insert_resource(Scoreboard {
            player_hp: PLAYER_HP,
            enemy_hp: ENEMY_HP,
        })
        .insert_resource(EnemyAttackTimer(Timer::from_seconds(
            ENEMY_ATTACK_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, apply_velocity)
        .add_systems(Update, move_player)
        .add_systems(Update, player_shoot)
        .add_systems(Update, move_enemy)
        .add_systems(Update, enemy_shoot)
        .add_systems(Update, bullet_collision)
        .add_systems(
            Update,
            (remove_bullet, update_scoreboard, bevy::window::close_on_esc),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Collider {
    pub name: String,
    pub hp: f32,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource)]
struct Scoreboard {
    player_hp: f32,
    enemy_hp: f32,
}

#[derive(Resource)]
struct EnemyAttackTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    let player_y = -WINDOW_HALF_SIZE.y + GAP_BETWEEN_PLAYER_AND_FLOOR;

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
        Collider {
            name: "player".to_string(),
            hp: PLAYER_HP,
        },
    ));

    // Enemy
    let enemy_y = WINDOW_HALF_SIZE.y - SCOREBOARD_SIZE.y - GAP_BETWEEN_ENEMY_AND_TOP;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(ENEMY_SIZE, 4).into())
                .into(),
            material: materials.add(ColorMaterial::from(ENEMY_COLOR)),
            transform: Transform::from_translation(Vec3::new(0., enemy_y, 0.)),
            ..default()
        },
        Enemy,
        Velocity(INITIAL_ENEMY_DIRECTION.normalize() * ENEMY_SPEED),
        Collider {
            name: "enemy".to_string(),
            hp: ENEMY_HP,
        },
    ));

    // Scoreboard
    let font_bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_closure = |font: Handle<Font>, text: String| -> TextSection {
        let style = TextStyle {
            font,
            font_size: SCOREBOARD_FONT_SIZE,
            color: TEXT_COLOR,
        };
        TextSection::new(text, style)
    };

    commands.spawn(
        TextBundle::from_sections([
            text_closure(font_bold.clone(), "Player: ".to_string()),
            text_closure(font_medium.clone(), PLAYER_HP.to_string()),
            text_closure(font_bold.clone(), ", Enemy: ".to_string()),
            text_closure(font_medium.clone(), ENEMY_HP.to_string()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(SCOREBOARD_TEXT_PADDING),
            left: Val::Px(SCOREBOARD_TEXT_PADDING),
            ..default()
        }),
    );
    // Scoreboard background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SCOREBOARD_COLOR,
            custom_size: Some(SCOREBOARD_SIZE),
            ..default()
        },
        transform: Transform::from_translation(
            Vec2::new(0.0, WINDOW_HALF_SIZE.y - SCOREBOARD_SIZE.y / 2.).extend(0.0),
        ),
        ..default()
    });
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.player_hp.to_string();
    text.sections[3].value = scoreboard.enemy_hp.to_string();
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut player_transform = player_query.single_mut();
    let mut direction = Vec2::ZERO;

    // Keyboard input
    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    } else if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.0;
    } else if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.0;
    }

    // Player x movement
    let new_player_position_x = player_transform.translation.x
        + direction.x * PLAYER_SPEED * time_step.period.as_secs_f32();
    let left_bound = -WINDOW_HALF_SIZE.x + PLAYER_SIZE / 2.0 + PLAYER_PADDING;
    let right_bound = WINDOW_HALF_SIZE.x - PLAYER_SIZE / 2.0 - PLAYER_PADDING;

    // Player y movement
    let new_player_position_y = player_transform.translation.y
        + direction.y * PLAYER_SPEED * time_step.period.as_secs_f32();
    let up_bound = -WINDOW_HALF_SIZE.y + PLAYER_SIZE / 2.0 + PLAYER_PADDING;
    let down_bound = WINDOW_HALF_SIZE.y - PLAYER_SIZE / 2.0 - PLAYER_PADDING - SCOREBOARD_SIZE.y;

    player_transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);
    player_transform.translation.y = new_player_position_y.clamp(up_bound, down_bound);
}

fn player_shoot(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();

    if keyboard_input.just_pressed(KeyCode::Space) {
        // Bullet
        let bullet_y = player_transform.translation.y + PLAYER_SIZE / 2.0 + BULLET_SIZE / 2.0;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BULLET_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
                transform: Transform::from_translation(
                    Vec2::new(player_transform.translation.x, bullet_y).extend(0.),
                ),
                ..default()
            },
            Bullet,
            Velocity(Vec2::new(0., 0.5) * BULLET_SPEED),
        ));
    }
}

fn move_enemy(mut enemy_query: Query<(&Transform, &mut Velocity), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    let (enemy_transform, mut enemy_velocity) = enemy_query.single_mut();
    let left_window_collision =
        WINDOW_HALF_SIZE.x < enemy_transform.translation.x + ENEMY_SIZE / 2.0 + 10.0;
    let right_window_collision =
        -WINDOW_HALF_SIZE.x > enemy_transform.translation.x - ENEMY_SIZE / 2.0 - 10.0;

    if left_window_collision || right_window_collision {
        enemy_velocity.x = -enemy_velocity.x;
    }
}

fn enemy_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
    mut timer: ResMut<EnemyAttackTimer>,
) {
    if enemy_query.is_empty() {
        return;
    }

    let enemy_transform = enemy_query.single();

    // Bullet
    let bullet_y = enemy_transform.translation.y - ENEMY_SIZE / 2.0 - BULLET_SIZE / 2.0;

    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BULLET_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(ENEMY_COLOR)),
                transform: Transform::from_translation(
                    Vec2::new(enemy_transform.translation.x, bullet_y).extend(0.),
                ),
                ..default()
            },
            Bullet,
            Velocity(Vec2::new(0., -0.5) * BULLET_SPEED),
        ));
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut collider_query: Query<(&mut Collider, Entity, &Transform), With<Collider>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for (mut collider, collider_entity, collider_transform) in collider_query.iter_mut() {
            let bullet_size = Vec2::new(BULLET_SIZE, BULLET_SIZE);
            let mut collider_size = Vec2::ZERO;

            if collider.name == "player".to_string() {
                collider_size = Vec2::new(PLAYER_SIZE, PLAYER_SIZE);
            } else if collider.name == "enemy".to_string() {
                collider_size = Vec2::new(ENEMY_SIZE, ENEMY_SIZE);
            }

            let collision = collide(
                bullet_transform.translation,
                bullet_size,
                collider_transform.translation,
                collider_size,
            );

            if let Some(..) = collision {
                commands.entity(bullet_entity).despawn();
                collider.hp -= 1.0;

                if collider.name == "player".to_string() {
                    scoreboard.player_hp -= 1.0;
                } else if collider.name == "enemy".to_string() {
                    scoreboard.enemy_hp -= 1.0;
                }

                if collider.hp <= 0.0 {
                    commands.entity(collider_entity).despawn();
                }
            }
        }
    }
}

fn remove_bullet(mut commands: Commands, bullet_query: Query<(Entity, &Transform), With<Bullet>>) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation;

        if bullet_pos.x < -WINDOW_HALF_SIZE.x
            || bullet_pos.x > WINDOW_HALF_SIZE.x
            || bullet_pos.y < -WINDOW_HALF_SIZE.y
            || bullet_pos.y > WINDOW_HALF_SIZE.y
        {
            commands.entity(bullet_entity).despawn();
        }
    }
}
