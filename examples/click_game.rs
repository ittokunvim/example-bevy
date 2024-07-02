use bevy::{
    input::keyboard::KeyboardInput, prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow,
};
use rand::distributions::{Distribution, Uniform};

const WINDOW_SIZE: Vec2 = Vec2::new(1080.0, 720.0);
const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

const BALL_COUNT: usize = 30;
const BALL_SIZE: Vec3 = Vec3::new(50.0, 50.0, 0.0);
const BALL_SPEED: f32 = 400.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const PRESSANYKEY_FONT_SIZE: f32 = 40.0;
const PRESSANYKEY_TEXT_PADDING: Val = Val::Px(20.0);

const BALL_COLOR: Color = Color::rgb(0.9, 0.3, 0.3);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const PRESSANYKEY_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Resource, Component)]
struct Scoreboard {
    ball_count: usize,
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
        .add_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(Scoreboard {
            ball_count: BALL_COUNT,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, mouse_click.run_if(in_state(AppState::InGame)))
        .add_systems(Update, check_for_collisions.run_if(in_state(AppState::InGame)))
        .add_systems(Update, apply_velocity.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_scoreboard.run_if(in_state(AppState::InGame)))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct PressAnyKey;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let die_width = Uniform::from(-WINDOW_SIZE.x / 2.0 + BALL_SIZE.x..WINDOW_SIZE.x / 2.0 - BALL_SIZE.x);
    let die_height = Uniform::from(-WINDOW_SIZE.y / 2.0 + BALL_SIZE.y..WINDOW_SIZE.y / 2.0 - BALL_SIZE.y);
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

    // Scoreboard
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Ball Count: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::new(
                BALL_COUNT.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCORE_COLOR,
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
        Scoreboard { ball_count: 0 },
    ));

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
}

fn press_any_key(
    mut keyboard_event: EventReader<KeyboardInput>,
    pressanykey_query: Query<Entity, With<PressAnyKey>>,
    mut commands: Commands,
    mut now_state: ResMut<State<AppState>>,
    mut inkey: ResMut<Input<KeyCode>>,
) {
    for _event in keyboard_event.read() {
        let pressanykey_entity = pressanykey_query.single();
        commands.entity(pressanykey_entity).despawn();

        *now_state = State::new(AppState::InGame);
        inkey.reset_all();
    }
}

fn mouse_click(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_event: Res<Input<MouseButton>>,
    balls_query: Query<(Entity, &Transform), With<Ball>>,
) {
    let window = window_query.single();

    if mouse_event.just_pressed(MouseButton::Left) {
        let mut cursor_position = window.cursor_position().unwrap();
        let window_center = Vec2::new(window.width() / 2., window.height() / 2.);
        cursor_position = Vec2::new(
            cursor_position.x - window_center.x,
            -cursor_position.y + window_center.y,
        );

        for (ball_entity, ball_transform) in balls_query.iter() {
            let ball_pos = ball_transform.translation.truncate();
            let distance = cursor_position.distance(ball_pos);
            if distance < BALL_SIZE.x - 10.0 {
                scoreboard.ball_count -= 1;
                commands.entity(ball_entity).despawn();
            }
        }
    }
}

fn check_for_collisions(mut balls_query: Query<(&mut Velocity, &Transform), With<Ball>>) {
    for (mut ball_velocity, ball_transform) in balls_query.iter_mut() {
        let ball_size = ball_transform.scale.truncate();

        let left_window_collision =
            WINDOW_SIZE.x / 2.0 < ball_transform.translation.x + ball_size.x / 2.0;
        let right_window_collision =
            -WINDOW_SIZE.x / 2.0 > ball_transform.translation.x - ball_size.x / 2.0;
        let top_window_collision =
            WINDOW_SIZE.y / 2.0 < ball_transform.translation.y + ball_size.y / 2.0;
        let bottom_window_collision =
            -WINDOW_SIZE.y / 2.0 > ball_transform.translation.y - ball_size.y / 2.0;

        if left_window_collision || right_window_collision {
            ball_velocity.x = -ball_velocity.x;
        }

        if top_window_collision || bottom_window_collision {
            ball_velocity.y = -ball_velocity.y;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut scoreboard_query: Query<&mut Text, With<Scoreboard>>) {
    let mut text = scoreboard_query.single_mut();
    text.sections[1].value = scoreboard.ball_count.to_string();
}
