use bevy::prelude::*;
use bevy::log::LogPlugin;

const GAMETITLE: &str = "Collision";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,collision=debug";
const BALL_COUNT: usize = 9;
const BALL_SIZE: f32 = 20.0;
const BALL_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BALL_MARGIN: f32 = 10.0;
const BALL_SPEED: f32 = 80.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.into(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(LogPlugin {
                filter: LOG_FILTER.into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            })
         )
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            check_wall_collisions,
            check_ball_collisions,
            apply_velocity,
        ))
        .run();
}

#[derive(Component, Debug)]
struct Ball;

#[derive(Component, Debug)]
struct Collision;

#[derive(Component, Debug, Deref, DerefMut)]
struct Velocity(Vec2);

/// 衝突判定を実装するためのセットアップ
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info_once!("setup");

    // カメラを生成
    commands.spawn(Camera2d::default());

    // 任意の数のボールを生成
    let shape = meshes.add(Circle::new(BALL_SIZE));
    let color = materials.add(BALL_COLOR);
    let init_x = (-BALL_SIZE * 2.0 - BALL_MARGIN) * (BALL_COUNT / 2) as f32;
    for i in 0..BALL_COUNT {
        let x = init_x + ((BALL_SIZE * 2.0 + BALL_MARGIN) * i as f32);
        let translation = Vec3::ZERO.with_x(x);
        let x_speed = if i <= 0 { BALL_SPEED } else { 0.0 };
        commands.spawn((
            Mesh2d(shape.clone()),
            MeshMaterial2d(color.clone()),
            Transform::from_translation(translation),
            Ball,
            Collision,
            Velocity(Vec2::ZERO.with_x(x_speed))
        ));
    }
}

/// 壁の衝突を判定する関数
fn check_wall_collisions(
    mut query: Query<(&mut Velocity, &Transform), With<Collision>>,
) {
    for (mut velocity, transform) in query.iter_mut() {
        let left_window_collision =
            WINDOW_SIZE.x / 2.0 < transform.translation.x + BALL_SIZE;
        let right_window_collision =
            -WINDOW_SIZE.x / 2.0 > transform.translation.x - BALL_SIZE;
        let top_window_collision =
            WINDOW_SIZE.y / 2.0 < transform.translation.y + BALL_SIZE;
        let bottom_window_collision =
            -WINDOW_SIZE.y / 2.0 > transform.translation.y - BALL_SIZE;

        // 衝突物が画面端に触れたら、衝突物の動きの向きを反転させる
        if left_window_collision
        || right_window_collision
        || top_window_collision
        || bottom_window_collision {
            debug!("wall collision!");
            if left_window_collision || right_window_collision { velocity.x = -velocity.x }
            if top_window_collision || bottom_window_collision { velocity.y = -velocity.y }
        }
    }
}

/// ボール同士の衝突を判定する関数
fn check_ball_collisions(
    mut query: Query<(&mut Velocity, &Transform), With<Collision>>,
    time_step: Res<Time<Fixed>>,
) {
    let mut combinations = query.iter_combinations_mut();
    let ball_size = BALL_SIZE * 2.0;
    while let Some([ball1, ball2]) = combinations.fetch_next() {
        let (mut velocity_1, transform_1) = ball1;
        let (mut velocity_2, transform_2) = ball2;
        let position_1 = transform_1.translation.truncate();
        let position_2 = transform_2.translation.truncate();
        let direction_1 = velocity_1.xy() * time_step.delta().as_secs_f32();
        let direction_2 = velocity_2.xy() * time_step.delta().as_secs_f32();
        let collision = (
        (position_1.x + direction_1.x * 2.0 - position_2.x - direction_2.x * 2.0).powi(2) +
        (position_1.y + direction_1.y * 2.0 - position_2.y - direction_2.y * 2.0).powi(2)
    ) <= ball_size.powi(2);

        // ボール同士が触れたら、当たったボールと当てられたボールの動きの向きを入れ替える
        if collision {
            debug!("ball collision!");
            velocity_1.x += (direction_2.x - direction_1.x) / time_step.delta().as_secs_f32();
            velocity_1.y += (direction_2.y - direction_1.y) / time_step.delta().as_secs_f32();
            velocity_2.x += (direction_1.x - direction_2.x) / time_step.delta().as_secs_f32();
            velocity_2.y += (direction_1.y - direction_2.y) / time_step.delta().as_secs_f32();
        }
    }
}

/// 速度を追加する関数
fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Ball>>,
    time_step: Res<Time<Fixed>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

