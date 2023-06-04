use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use rand::distributions::{Distribution, Uniform};

const BALL_COUNT: u32 = 100;
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;

const WALL_THICKNESS: f32 = 10.0;
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BALL_COLOR: Color = Color::rgb(0.9, 0.3, 0.3);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_startup_system(setup)
        .add_systems((apply_velocity, check_for_collisions, mouse_click))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let area_height = TOP_WALL - BOTTOM_WALL;
        let area_width = RIGHT_WALL - LEFT_WALL;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, area_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(area_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(0.0),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    let mut rng = rand::thread_rng();
    let die_width = Uniform::from(LEFT_WALL + BALL_SIZE.x..RIGHT_WALL - BALL_SIZE.x);
    let die_height = Uniform::from(BOTTOM_WALL + BALL_SIZE.y..TOP_WALL - BALL_SIZE.y);
    let die_velocity = Uniform::from(-0.5..0.5);

    for _ in 0..BALL_COUNT {
        let ball_pos_x = die_width.sample(&mut rng);
        let ball_pos_y = die_height.sample(&mut rng);
        let ball_velocity_x = die_velocity.sample(&mut rng);
        let ball_velocity_y = die_velocity.sample(&mut rng);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(BALL_COLOR)),
                transform: Transform::from_translation(Vec3::new(ball_pos_x, ball_pos_y, 1.0))
                    .with_scale(BALL_SIZE),
                ..default()
            },
            Ball,
            Velocity(Vec2::new(ball_velocity_x, ball_velocity_y) * BALL_SPEED),
        ));
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn check_for_collisions(
    mut balls_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    for (mut ball_velocity, ball_transform) in balls_query.iter_mut() {
        let ball_size = ball_transform.scale.truncate();

        for transform in &collider_query {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}

fn mouse_click(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_event: Res<Input<MouseButton>>,
) {
    let window = window_query.single();

    if mouse_event.just_pressed(MouseButton::Left) {
        let mut cursor_position = window.cursor_position().unwrap();
        let window_center = Vec2::new(window.width() / 2., window.height() / 2.);
        cursor_position = Vec2::new(
            cursor_position.x - window_center.x,
            cursor_position.y - window_center.y,
        );
        println!("Mouse click at {:?}", cursor_position);
    }
}
