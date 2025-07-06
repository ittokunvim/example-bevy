use bevy::{
    prelude::*,
    log::LogPlugin,
    asset::AssetMetaCheck,
};

const GAMETITLE: &str = "スコアボード";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,scoreboard=debug";
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const DESCRIPTION_TEXT: &str = "Aキーを押すとスコアが大きくなる";
const SCORE_TEXT: &str = "スコア：";
const TEXT_SIZE: f32 = 30.0;
const TEXT_PADDING: f32 = 15.0;
const KEY_ADD_SCORE: KeyCode = KeyCode::KeyA;

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
        .add_systems(Update, update)
        .run()
    ;
}

#[derive(Component, Debug, Deref, DerefMut)]
struct Score(usize);

/// スコアボードのセットアップを行う関数
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // カメラを生成
    commands.spawn(Camera2d::default());

    // 説明を追加
    let font = asset_server.load(PATH_FONT);
    commands.spawn((
        Text2d::new(DESCRIPTION_TEXT),
        TextFont::from_font(font.clone()),
    ));

    // スコアボードを生成
    let text_font = TextFont {
            font,
            font_size: TEXT_SIZE,
            ..Default::default()
    };
    commands.spawn((
        Text::new(SCORE_TEXT),
        text_font.clone(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(TEXT_PADDING),
            left: Val::Px(TEXT_PADDING),
            ..Default::default()
        },
        children![(
            TextSpan::new("0"),
            text_font.clone(),
            Score(0),
        )],
    ));
}

/// スコアボードの更新を行う関数
fn update(
    mut query: Query<(&mut Score, &mut TextSpan), With<Score>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Aキーが押されたらスコアを加算する
    if keyboard_input.just_pressed(KEY_ADD_SCORE) {
        for (mut score, mut span) in &mut query {
            **score += 1;
            **span = format!("{}", **score);
        }
    }
}

