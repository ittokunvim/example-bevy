use bevy::prelude::*;

const SLIDER_SIZE: Vec2 = Vec2::new(500.0, 50.0);

const CUE_SIZE: Vec2 = Vec2::new(5.0, 50.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Slider
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            custom_size: Some(SLIDER_SIZE),
            ..default()
        },
        ..default()
    });

    // Cue
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::YELLOW,
            custom_size: Some(CUE_SIZE),
            ..default()
        },
        ..default()
    });
}
