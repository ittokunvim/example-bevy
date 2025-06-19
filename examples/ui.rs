use bevy::prelude::*;
use bevy::log::LogPlugin;

const GAMETITLE: &str = "UI (User Interface)";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,ui=debug";

const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_IMAGE_FONTAWESOME: &str = "images/fontawesome.png";

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
        .run();
}

