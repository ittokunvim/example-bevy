use bevy::prelude::*;

const WINDOW_SIZE: Vec2 = Vec2::new(900.0, 600.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_system(bevy::window::close_on_esc)
        .run();
}

