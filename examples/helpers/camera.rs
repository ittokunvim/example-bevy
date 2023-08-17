use bevy::prelude::*;

#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
