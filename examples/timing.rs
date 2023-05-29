use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const SLIDER_SIZE: Vec2 = Vec2::new(500.0, 50.0);

const REFLECTOR_SIZE: Vec2 = Vec2::new(1.0, 50.0);

const CUE_SIZE: Vec2 = Vec2::new(5.0, 50.0);
const CUE_SPEED: f32 = 500.0;
const INITIAL_CUE_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const PERFECT_TIMING_RANGE: f32 = 10.0;
const GOOD_TIMING_RANGE: f32 = 50.0;
const OK_TIMING_RANGE: f32 = 150.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_event::<CollisionEvent>()
        .add_event::<TimingEvent>()
        .add_startup_system(setup)
        .add_systems((
            check_for_collisions,
            apply_velocity.before(check_for_collisions),
            decide_timing
                .before(check_for_collisions)
                .after(apply_velocity),
            play_timing_sound.after(check_for_collisions),
        ))
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

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

#[derive(Default)]
struct TimingEvent;

#[derive(Resource)]
struct TimingSound(Handle<AudioSource>);

#[derive(Resource)]
struct Scoreboard {
    score: isize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Sound
    let cue_timing_sound = asset_server.load("sounds/timing_decide.ogg");
    commands.insert_resource(TimingSound(cue_timing_sound));

    // Slider
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            custom_size: Some(SLIDER_SIZE),
            ..default()
        },
        ..default()
    });

    // Slider ok timing range
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(OK_TIMING_RANGE * 2.0, SLIDER_SIZE.y)),
            ..default()
        },
        ..default()
    });

    // Slider good timing range
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(GOOD_TIMING_RANGE * 2.0, SLIDER_SIZE.y)),
            ..default()
        },
        ..default()
    });

    // Slider parfect timing range
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(PERFECT_TIMING_RANGE * 2.0, SLIDER_SIZE.y)),
            ..default()
        },
        ..default()
    });

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

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: Color::GRAY,
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

fn decide_timing(
    keyboard_input: Res<Input<KeyCode>>,
    mut scoreboard: ResMut<Scoreboard>,
    query: Query<&Transform, With<Cue>>,
    mut timing_events: EventWriter<TimingEvent>,
) {
    let cue_transform = query.single();

    if keyboard_input.just_pressed(KeyCode::Space) {
        // Sends a timing event so that other systems can react to the timing
        timing_events.send_default();

        let cue_translation_x = cue_transform.translation.x;
        println!("{}", cue_translation_x);

        if cue_translation_x < PERFECT_TIMING_RANGE && cue_translation_x > -PERFECT_TIMING_RANGE {
            scoreboard.score += 100;
        } else if cue_translation_x < GOOD_TIMING_RANGE && cue_translation_x > -GOOD_TIMING_RANGE {
            scoreboard.score += 50;
        } else if cue_translation_x < OK_TIMING_RANGE && cue_translation_x > -OK_TIMING_RANGE {
            scoreboard.score += 10;
        } else {
            scoreboard.score += -100;
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn play_timing_sound(
    mut timing_events: EventReader<TimingEvent>,
    audio: Res<Audio>,
    sound: Res<TimingSound>,
) {
    // Play a sound once per frame if a timing occurred.
    if !timing_events.is_empty() {
        // This prevents events staying active on the next frame.
        timing_events.clear();
        audio.play(sound.0.clone());
    }
}
