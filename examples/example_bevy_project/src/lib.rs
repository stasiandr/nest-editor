use bevy::prelude::*;

#[nest_editor_macro::app_builder]
pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Update, move_cube);
    app
}

pub fn move_cube(
        time: Res<Time>, 
        mut query: Query<(&mut Transform, &Name)>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    
    for (mut transform, name) in query.iter_mut() {
        if name.as_str() != "Cube" {
            continue;
        }

        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction -= Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::X;
        }

        if direction.length() > 0.0 {
            transform.translation += direction.normalize() * time.delta_secs() * 2.0;
        }
    }
}