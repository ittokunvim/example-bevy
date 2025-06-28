use bevy::{
    prelude::*,
    log::LogPlugin,
    asset::AssetMetaCheck,
};

const GAMETITLE: &str = "スプライトシート";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,spritesheet=debug";

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
        .run()
    ;
}
