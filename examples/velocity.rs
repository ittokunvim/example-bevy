use bevy::prelude::*;
use bevy::log::LogPlugin;

const GAMETITLE: &str = "Velocity";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,velocity=debug";
const SQUARE_SIZE: f32 = 40.0;
const SQUARE_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const SQUARE_SPEED: f32 = 80.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
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
        .add_systems(Startup, setup)
        .add_systems(Update, (
            apply_velocity,
            check_wall_collisions,
        ))
        .run();
}

/// 速度を管理するコンポーネント
#[derive(Component, Debug, Deref, DerefMut)]
struct Velocity(Vec2);

/// 衝突を管理するコンポーネント
#[derive(Component, Debug)]
struct Collision;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info_once!("setup");

    // カメラを生成
    commands.spawn(Camera2d::default());

    // 四角形を生成
    let shape = meshes.add(Rectangle::new(SQUARE_SIZE, SQUARE_SIZE));
    let color = materials.add(SQUARE_COLOR);
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(color),
        Velocity(Vec2::new(SQUARE_SPEED, SQUARE_SPEED)),
        Collision,
    ));
}

/// 速度を追加する関数
fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Velocity>>,
    time_step: Res<Time<Fixed>>,
) {
    info_once!("apply_velocity");

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

/// 壁の衝突を判定する関数
fn check_wall_collisions(
    mut query: Query<(&mut Velocity, &Transform), With<Collision>>,
) {
    info_once!("check_wall_collisions");

    for (mut velocity, transform) in query.iter_mut() {
        let left_window_collision =
            WINDOW_SIZE.x / 2.0 < transform.translation.x + SQUARE_SIZE / 2.0;
        let right_window_collision =
            -WINDOW_SIZE.x / 2.0 > transform.translation.x - SQUARE_SIZE / 2.0;
        let top_window_collision =
            WINDOW_SIZE.y / 2.0 < transform.translation.y + SQUARE_SIZE / 2.0;
        let bottom_window_collision =
            -WINDOW_SIZE.y / 2.0 > transform.translation.y - SQUARE_SIZE / 2.0;

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

