use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::color::palettes::basic::*;

const GAMETITLE: &str = "オーディオ";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,audio=debug";

const PATH_SOUND_CLICK: &str = "sounds/click.ogg";
const PATH_SOUND_BGM: &str = "sounds/bgm.ogg";
const BUTTON_SIZE: Vec2 = Vec2::new(80.0, 40.0);
const BUTTON_FONT_SIZE: f32 = 20.0;
const BUTTON_BORDER_SIZE: f32 = 2.0;
const BUTTON_BORDER_RADIUS: f32 = 5.0;
const BUTTON_GAP: f32 = 10.0;
const BUTTON_PLAY_TEXT: &str = "Play.";
const BUTTON_PAUSE_TEXT: &str = "Pause";
const BUTTON_MUTE_TEXT: &str = "Mute.";

#[derive(Resource, Deref, DerefMut)]
struct ClickSound(Handle<AudioSource>);

#[derive(Component, Debug)]
struct Bgm;

#[derive(Component, Debug)]
struct PlayButton;

#[derive(Component, Debug)]
struct PauseButton;

#[derive(Component, Debug)]
struct MuteButton;

/// ここでは、以下の機能の実装の例が書かれています。
/// - クリック音
/// - BGMの再生
/// - BGMの停止
/// - BGMの一時停止
/// - BGMのミュート
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
            play_bgm,
            pause_bgm,
            mute_bgm,
            play_clicksound,
        ))
        .run();
}

/// 音源を再生するためのセットアップを行う
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    // クリック音をリソースに登録
    let sound = asset_server.load(PATH_SOUND_CLICK);
    commands.insert_resource(ClickSound(sound));

    // BGMを生成
    let sound = asset_server.load(PATH_SOUND_BGM);
    commands.spawn((
        AudioPlayer::new(sound),
        PlaybackSettings::LOOP,
        Bgm,
    ));

    // カメラを生成
    commands.spawn(Camera2d::default());

    // ボタンリストを生成
    let button_node = (
        Button,
        Node {
            width: Val::Px(BUTTON_SIZE.x),
            height: Val::Px(BUTTON_SIZE.y),
            border: UiRect::all(Val::Px(BUTTON_BORDER_SIZE)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BorderColor(GRAY.into()),
        BorderRadius::all(Val::Px(BUTTON_BORDER_RADIUS))
    );
    let closure_text_node = |text: &str| {
        (
            Text::new(text),
            TextFont::from_font_size(BUTTON_FONT_SIZE),
            TextColor(WHITE.into()),
        )
    };
    // ボタンルートを作成
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        display: Display::Flex,
        column_gap: Val::Px(BUTTON_GAP),
        ..Default::default()
    })
    // プレイボタンを生成
    .with_children(|parent| {
        parent.spawn((PlayButton, button_node.clone()))
            .with_child(closure_text_node(BUTTON_PLAY_TEXT));
    })
    // ポーズボタンを生成
    .with_children(|parent| {
        parent.spawn((PauseButton, button_node.clone()))
            .with_child(closure_text_node(BUTTON_PAUSE_TEXT));
    })
    // ミュートボタンを生成
    .with_children(|parent| {
        parent.spawn((MuteButton, button_node.clone()))
            .with_child(closure_text_node(BUTTON_MUTE_TEXT));
    });
}

/// 再生ボタンが押されたらBGMを再生する
fn play_bgm(
    playbutton_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    bgm_query: Query<&AudioSink, With<Bgm>>,
) {
    info_once!("play_bgm");

    for interaction in &playbutton_query {
        if *interaction == Interaction::Pressed {
            if let Ok(audio) = bgm_query.get_single() {
                debug!("play bgm");
                audio.play();
            }
        }
    }
}

/// 一時停止ボタンが押されたらBGMを一時停止する
fn pause_bgm(
    pausebutton_query: Query<&Interaction, (Changed<Interaction>, With<PauseButton>)>,
    bgm_query: Query<&AudioSink, With<Bgm>>,
) {
    info_once!("pause_bgm");

    for interaction in &pausebutton_query {
        if *interaction == Interaction::Pressed {
            if let Ok(audio) = bgm_query.get_single() {
                debug!("pause bgm");
                audio.pause();
            }
        }
    }
}

/// ミュートボタンが押されたらBGMをミュートする
fn mute_bgm(
    mutebutton_query: Query<&Interaction, (Changed<Interaction>, With<MuteButton>)>,
    bgm_query: Query<&AudioSink, With<Bgm>>,
) {
    info_once!("mute_bgm");

    for interaction in &mutebutton_query {
        if *interaction == Interaction::Pressed {
            if let Ok(audio) = bgm_query.get_single() {
                if audio.volume() < 0.01 {
                    debug!("unmute bgm");
                    audio.set_volume(1.0);
                } else {
                    debug!("mute bgm");
                    audio.set_volume(0.0);
                }
            }
        }
    }
}

/// 左クリックでクリック音が再生される
fn play_clicksound(
    mut commands: Commands,
    mouse_events: Res<ButtonInput<MouseButton>>,
    sound: Res<ClickSound>,
) {
    info_once!("play_clicksound");

    if mouse_events.just_pressed(MouseButton::Left) {
        debug!("play click sound");
        commands.spawn(AudioPlayer(sound.clone()));
    }
}

