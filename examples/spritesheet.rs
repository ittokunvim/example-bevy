use std::time::Duration;

use bevy::{
    prelude::*,
    log::LogPlugin,
    asset::AssetMetaCheck,
};

const GAMETITLE: &str = "スプライトシート";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,spritesheet=debug";
const PATH_IMAGES_SPRITESHEET: &str = "images/spritesheet.png";

const IMAGE_SIZE: u32 = 32;
const SIZE: f32 = 64.0;
const IMAGE_COLUMN: u32 = 6;
const IMAGE_ROW: u32 = 6;

const KEY_SPRITESHEET_IDLE: KeyCode = KeyCode::KeyI;
const KEY_SPRITESHEET_RUN: KeyCode = KeyCode::KeyB;
const KEY_SPRITESHEET_CLIMB: KeyCode = KeyCode::KeyW;
const KEY_SPRITESHEET_CROUCH: KeyCode = KeyCode::KeyS;
const KEY_SPRITESHEET_HURT: KeyCode = KeyCode::KeyH;
const KEY_SPRITESHEET_JUMP: KeyCode = KeyCode::KeyJ;

const IDLE_INDICES: (usize, usize) = (0, 3);
const RUN_INDICES: (usize, usize) = (6, 11);
const CLIMB_INDICES: (usize, usize) = (12, 15);
const CROUCH_INDICES: (usize, usize) = (18, 20);
const HURT_INDICES: (usize, usize) = (24, 25);
const JUMP_INDICES: (usize, usize) = (30, 31);

const IDLE_FPS: u8 = 4;
const RUN_FPS: u8 = 6;
const CLIMB_FPS: u8 = 4;
const CROUCH_FPS: u8 = 3;
const HURT_FPS: u8 = 2;
const JUMP_FPS: u8 = 2;

/// ここではスプライト画像によるアニメーションの例が書かれています
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
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                filter: LOG_FILTER.into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
        )
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            animation,
            idle_events,
            run_events,
            climb_events,
            crouch_events,
            hurt_events,
            jump_events,
        ))
        .run()
    ;
}

/// アニメーションを設定するコンポーネント
#[derive(Component, Debug)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    /// アニメーションの初期化を行う関数
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    /// FPSからタイマーを返す関数
    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}

/// スプライトシートのセットアップを行う関数
fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    // カメラを生成
    commands.spawn(Camera2d::default());

    // スプライトシートを生成
    let texture = asset_server.load(PATH_IMAGES_SPRITESHEET);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(IMAGE_SIZE),
        IMAGE_COLUMN,
        IMAGE_ROW,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(IDLE_INDICES.0, IDLE_INDICES.1, IDLE_FPS);
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: IDLE_INDICES.0,
            }),
            custom_size: Some(Vec2::splat(SIZE)),
            ..Default::default()
        },
        animation_config,
    ));

    // テキストを生成
    let text = format!(
        "Idle: {:?}\nRun: {:?}\nClimb: {:?}\nCrouch: {:?}\nHurt: {:?}\nJump: {:?}",
        KEY_SPRITESHEET_IDLE,
        KEY_SPRITESHEET_RUN,
        KEY_SPRITESHEET_CLIMB,
        KEY_SPRITESHEET_CROUCH,
        KEY_SPRITESHEET_HURT,
        KEY_SPRITESHEET_JUMP,
    );
    commands.spawn(Text::new(text));
}

/// スプライトシートのアニメーションを行う関数
fn animation(
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    time: Res<Time>,
) {
    info_once!("animation");

    for (mut config, mut sprite) in &mut query {
        // タイマーを進める
        config.frame_timer.tick(time.delta());

        // タイマーが終わったら、スプライト画像を1コマ進める
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

/// 対応のキーが押されたらスプライトシートをアイドルで描画する関数
fn idle_events(
    query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("idle_events");

    handle_animation_event(
        query,
        keyboard_input,
        KEY_SPRITESHEET_IDLE,
        IDLE_INDICES,
        IDLE_FPS,
    );
}

/// 対応のキーが押されたらスプライトシートをランで描画する関数
fn run_events(
    query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("run_events");

    handle_animation_event(
        query,
        keyboard_input,
        KEY_SPRITESHEET_RUN,
        RUN_INDICES,
        RUN_FPS,
    );
}

/// 対応のキーが押されたらスプライトシートを登るで描画する関数
fn climb_events(
    query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("climb_events");

    handle_animation_event(
        query,
        keyboard_input,
        KEY_SPRITESHEET_CLIMB,
        CLIMB_INDICES,
        CLIMB_FPS,
    );
}

/// 対応のキーが押されたらスプライトシートをしゃがむで描画する関数
fn crouch_events(
    query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("crouch_events");

    handle_animation_event(
        query,
        keyboard_input,
        KEY_SPRITESHEET_CROUCH,
        CROUCH_INDICES,
        CROUCH_FPS,
    );
}

/// 対応のキーが押されたらスプライトシートをダメージで描画する関数
fn hurt_events(
    query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("hurt_events");

    handle_animation_event(
        query,
        keyboard_input,
        KEY_SPRITESHEET_HURT,
        HURT_INDICES,
        HURT_FPS,
    );
}

/// 対応のキーが押されたらスプライトシートをジャンプで描画する関数
fn jump_events(
    query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("jump_events");

    handle_animation_event(
        query,
        keyboard_input,
        KEY_SPRITESHEET_JUMP,
        JUMP_INDICES,
        JUMP_FPS,
    );
}
 
/// 渡されたキーに応じて、アニメーションの変更を行う関数
fn handle_animation_event(
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key: KeyCode,
    indices: (usize, usize),
    fps: u8,
) {
    if keyboard_input.just_pressed(key) {
        let (first, last) = indices;
        for (mut config, mut sprite) in &mut query {
            // タイマーをリセット
            config.frame_timer.reset();
            // アニメーション設定を更新
            *config = AnimationConfig::new(first, last, fps);
            // スプライトの描画を更新
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = first;
            }
        }
    }
}

