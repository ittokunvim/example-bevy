use bevy::prelude::*;
use bevy::log::LogPlugin;

const GAMETITLE: &str = "UI (User Interface)";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,ui=debug";

const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_IMAGE_FONTAWESOME: &str = "images/fontawesome.png";

const ROOT_WIDTH: Val = Val::Percent(100.0);
const ROOT_HEIGHT: Val = Val::Percent(100.0);

const BOARD_SIZE: Vec2 = Vec2::new(360.0, 270.0);
const BOARD_LEFT: Val = Val::Px(WINDOW_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0);
const BOARD_TOP: Val = Val::Px(WINDOW_SIZE.y / 2.0 - BOARD_SIZE.y / 2.0);
const BOARD_PADDING: Val = Val::Px(16.0);
const BOARD_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

const ICON_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const BUTTON_WIDTH: Val = Val::Px(ICON_SIZE.x * 2.0);
const BUTTON_HEIGHT: Val = Val::Px(ICON_SIZE.y * 2.0);
const ICON_COLOR_PRESS: Color = Color::srgb(0.3, 0.3, 0.3);
const ICON_COLOR_HOVER: Color = Color::srgb(0.5, 0.5, 0.5);

const TEXT_FONT_SIZE: f32 = 24.0;
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

const BORDER_SIZE: Val = Val::Px(4.0);
const BORDER_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const BORDER_RADIUS: Val = Val::Px(10.0);

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
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

/// UIのセットアップを行う関数
/// 構造：
/// * root
///   * board
///     * title
///     * button
///       * icon
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    // カメラを生成
    commands.spawn(Camera2d::default());

    // UIを生成
    let font = asset_server.load(PATH_FONT);
    let image = asset_server.load(PATH_IMAGE_FONTAWESOME);
    commands.spawn((
       Node {
            width: ROOT_WIDTH,
            height: ROOT_HEIGHT,
            ..Default::default()
        },
        children![(
            Node {
                width: Val::Px(BOARD_SIZE.x),
                height: Val::Px(BOARD_SIZE.y),
                border: UiRect::all(BORDER_SIZE),
                position_type: PositionType::Absolute,
                left: BOARD_LEFT,
                top: BOARD_TOP,
                padding: UiRect::all(BOARD_PADDING),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(BOARD_COLOR),
            BorderColor(BORDER_COLOR),
            BorderRadius::all(BORDER_RADIUS),
            children![(
                Text::new(GAMETITLE),
                TextFont {
                    font: font.clone(),
                    font_size: TEXT_FONT_SIZE,
                    ..Default::default()
                },
                TextColor(TEXT_COLOR),
            ),(
                Button,
                Node {
                    width: BUTTON_WIDTH,
                    height: BUTTON_HEIGHT,
                    border: UiRect::all(BORDER_SIZE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BorderColor(BORDER_COLOR),
                BorderRadius::all(BORDER_RADIUS),
                children![(
                    ImageNode::new(image.clone()),
                    Node {
                        width: Val::Px(ICON_SIZE.x),
                        height: Val::Px(ICON_SIZE.y),
                        ..Default::default()
                    },
                )],
            )],
        )]
    ));
}

/// ボタンが押された時の処理を決める関数
fn button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    info_once!("button_system");

    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = ICON_COLOR_PRESS.into();
            }
            Interaction::Hovered => {
                *color = ICON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *color = BOARD_COLOR.into();
            }
        }
    }
}
