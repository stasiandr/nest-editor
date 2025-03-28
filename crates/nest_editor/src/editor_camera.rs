use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasing, prelude::*};


#[derive(Component, Debug, Default)]
pub struct EditorCamera;

pub fn spawn_editor_camera (
    mut commands: Commands,
    q: Query<(Entity, &bevy::window::Window)>,
)
{
    let window = q.single().0;
    commands.spawn((
        EditorCamera,
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            hdr: true,
            target: bevy::render::camera::RenderTarget::Window(bevy::window::WindowRef::Entity(window)),
            ..Default::default()
        },
        bevy::pbr::ScreenSpaceAmbientOcclusion::default(),
        TemporalAntiAliasing::default(),
        bevy::core_pipeline::tonemapping::Tonemapping::BlenderFilmic,
        bevy::core_pipeline::bloom::Bloom::NATURAL,
        bevy::core_pipeline::motion_blur::MotionBlur {
            shutter_angle: 0.7,
            samples: 3,
        },
        Name::new("EditorCamera"),
    ));
}

pub fn editor_camera_controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mut mouse: EventReader<CursorMoved>,
    mut camera: Query<&mut Transform, With<EditorCamera>>,
) {
    if !mouse_btn.pressed(MouseButton::Right) {
        return;
    }

    let rot_mod = 15.0 * time.delta_secs();
    let move_mod = 5.0 * time.delta_secs();

    for mut transform in camera.iter_mut() {

        for delta in mouse.read().flat_map(|e| e.delta) {
            let left = transform.left();
            transform.rotate_axis(left, delta.y.to_radians() * rot_mod);
            transform.rotate_axis(Dir3::NEG_Y, delta.x.to_radians() * rot_mod);
        }

        let right = transform.right();
        let forward = transform.forward();
        let up = transform.up();

        if keys.pressed(KeyCode::KeyW) {
            transform.translation += forward * move_mod;        
        }

        if keys.pressed(KeyCode::KeyS) {
            transform.translation -= forward * move_mod;        
        }

        if keys.pressed(KeyCode::KeyA) {
            transform.translation -= right * move_mod;        
        }

        if keys.pressed(KeyCode::KeyD) {
            transform.translation += right * move_mod;        
        }

        if keys.pressed(KeyCode::KeyE) {
            transform.translation += up * move_mod;        
        }

        if keys.pressed(KeyCode::KeyQ) {
            transform.translation -= up * move_mod;        
        }
    }
} 