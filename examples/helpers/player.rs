use bevy::prelude::*;

#[allow(dead_code)]
pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction = Vec2::ZERO;

    // Keyboard input
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y += 1.0;
    }

    // Player x movement
    let new_player_position_x = player_transform.translation.x
        + direction.x * PLAYER_SPEED * time_step.period.as_secs_f32();
    let x_bound = WINDOW_SIZE.x / 2.0 - PLAYER_SIZE.x;

    // Player y movement
    let new_player_position_y = player_transform.translation.y
        + direction.y * PLAYER_SPEED * time_step.period.as_secs_f32();
    let y_bound = WINDOW_SIZE.y / 2.0 - PLAYER_SIZE.y;

    player_transform.translation.x = new_player_position_x.clamp(-x_bound, x_bound);
    player_transform.translation.y = new_player_position_y.clamp(-y_bound, y_bound);
}
