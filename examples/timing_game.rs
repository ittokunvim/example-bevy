use bevy::{
    audio::Volume,
    input::keyboard::KeyboardInput,
    prelude::*,
};

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const BACKGROUND_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

const SLIDER_SIZE: Vec2 = Vec2::new(500.0, 50.0);
const SLIDER_DEFAULT_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const SLIDER_DEFAULT_POINTS: isize = -100;

const SLIDER_OK_RANGE: f32 = 100.0;
const SLIDER_OK_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const SLIDER_OK_POINTS: isize = 10;

const SLIDER_GOOD_RANGE: f32 = 60.0;
const SLIDER_GOOD_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
const SLIDER_GOOD_POINTS: isize = 50;

const SLIDER_PERFECT_RANGE: f32 = 20.0;
const SLIDER_PERFECT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const SLIDER_PERFECT_POINTS: isize = 100;

const CUE_SIZE: Vec2 = Vec2::new(5.0, 50.0);
const CUE_SPEED: f32 = 500.0;
const INITIAL_CUE_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const CUE_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const PRESSANYKEY_FONT_SIZE: f32 = 40.0;
const PRESSANYKEY_TEXT_PADDING: Val = Val::Px(20.0);
const PRESSANYKEY_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Resource, Component)]
struct Scoreboard {
    score: isize,
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
        .init_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Startup, setup)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, decide_timing.run_if(in_state(AppState::InGame)))
        .add_systems(Update, apply_velocity.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_scoreboard.run_if(in_state(AppState::InGame)))
        .run();
}

#[derive(Component)]
struct PressAnyKey;

#[derive(Component)]
struct Cue;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Press any key
    commands.spawn((
        TextBundle::from_section(
            "Press Any Key ...",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: PRESSANYKEY_FONT_SIZE,
                color: PRESSANYKEY_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: PRESSANYKEY_TEXT_PADDING,
            right: PRESSANYKEY_TEXT_PADDING,
            ..default()
        }),
        PressAnyKey,
    ));

    // Slider
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SLIDER_DEFAULT_COLOR,
            custom_size: Some(SLIDER_SIZE),
            ..default()
        },
        ..default()
    });

    [
        (SLIDER_OK_COLOR, SLIDER_OK_RANGE),
        (SLIDER_GOOD_COLOR, SLIDER_GOOD_RANGE),
        (SLIDER_PERFECT_COLOR, SLIDER_PERFECT_RANGE),
    ]
        .iter()
        .for_each(|(color, range)| {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: *color,
                    custom_size: Some(Vec2::new(range * 2.0, SLIDER_SIZE.y)),
                    ..default()
                },
                ..default()
            });

        });

    // Cue
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: CUE_COLOR,
                custom_size: Some(CUE_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Cue,
        Velocity(INITIAL_CUE_DIRECTION.normalize() * CUE_SPEED),
    ));

    // Scoreboard
    commands.spawn((
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
            TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: Color::BLACK,
                    ..default()
                },
            ),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            }),
        Scoreboard { score: 0 },
    ));
}

fn press_any_key(
    mut keyboard_event: EventReader<KeyboardInput>,
    pressanykey_query: Query<Entity, With<PressAnyKey>>,
    mut commands: Commands,
    mut now_state: ResMut<State<AppState>>,
    mut inkey: ResMut<ButtonInput<KeyCode>>,
) {
    for _event in keyboard_event.read() {
        let pressanykey_entity = pressanykey_query.single();
        commands.entity(pressanykey_entity).despawn();

        *now_state = State::new(AppState::InGame);
        inkey.reset_all();
    }
}

fn decide_timing(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scoreboard: ResMut<Scoreboard>,
    cue_query: Query<&Transform, With<Cue>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let cue_transform = cue_query.single();

    if keyboard_input.just_pressed(KeyCode::Space) {
        // Sends a timing event so that other systems can react to the timing
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/timing.ogg"),
            settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.5)),
        });

        let cue_translation_x = cue_transform.translation.x;

        if cue_translation_x < SLIDER_PERFECT_RANGE && cue_translation_x > -SLIDER_PERFECT_RANGE {
            scoreboard.score += SLIDER_PERFECT_POINTS;
        } else if cue_translation_x < SLIDER_GOOD_RANGE && cue_translation_x > -SLIDER_GOOD_RANGE {
            scoreboard.score += SLIDER_GOOD_POINTS;
        } else if cue_translation_x < SLIDER_OK_RANGE && cue_translation_x > -SLIDER_OK_RANGE {
            scoreboard.score += SLIDER_OK_POINTS;
        } else {
            scoreboard.score += SLIDER_DEFAULT_POINTS;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>, time_step: Res<Time<Fixed>>) {
    for (mut transform, mut velocity) in &mut query {
        let transform_x = transform.translation.x;
        if transform_x >= SLIDER_SIZE.x / 2.0 || transform_x <= -SLIDER_SIZE.x / 2.0 {
            velocity.x = -velocity.x;
        }
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
    }
}

fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut scoreboard_query: Query<&mut Text, With<Scoreboard>>,
) {
    let mut text = scoreboard_query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
