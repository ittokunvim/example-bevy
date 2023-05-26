use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const SLIDER_SIZE: Vec2 = Vec2::new(500.0, 50.0);

const REFLECTOR_SIZE: Vec2 = Vec2::new(1.0, 50.0);

const CUE_SIZE: Vec2 = Vec2::new(5.0, 50.0);
const CUE_SPEED: f32 = 100.0;
const INITIAL_CUE_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_event::<CollisionEvent>()
        .add_startup_system(setup)
        .add_systems((
            check_for_collisions,
            apply_velocity.before(check_for_collisions),
        ))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Slider;

#[derive(Component)]
struct Cue;

#[derive(Component)]
struct Reflector;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Slider
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(SLIDER_SIZE),
                ..default()
            },
            ..default()
        },
        Slider,
    ));

    let refrector_sprite = |slider_pos_x: f32| SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(REFLECTOR_SIZE),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-slider_pos_x / 2.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    };

    // left reflector
    commands.spawn((refrector_sprite(-SLIDER_SIZE.x), Reflector, Collider));

    // right reflector
    commands.spawn((refrector_sprite(SLIDER_SIZE.x), Reflector, Collider));

    // Cue
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(CUE_SIZE),
                ..default()
            },
            ..default()
        },
        Cue,
        Velocity(INITIAL_CUE_DIRECTION.normalize() * CUE_SPEED),
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
    }
}

fn check_for_collisions(
    mut cue_query: Query<(&mut Velocity, &Transform), With<Cue>>,
    collider_query: Query<&Transform, (With<Collider>, Without<Cue>)>,
) {
    let (mut cue_velocity, cue_transform) = cue_query.single_mut();

    // check collision with reflectors
    for transform in &collider_query {
        let collision = collide(
            cue_transform.translation,
            CUE_SIZE,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            let reflect_x = match collision {
                Collision::Left => cue_velocity.x > 0.0,
                Collision::Right => cue_velocity.x < 0.0,
                _ => false,
            };

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                cue_velocity.x = -cue_velocity.x;
            }
        }
    }
}
