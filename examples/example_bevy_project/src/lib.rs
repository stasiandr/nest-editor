use bevy::{prelude::*, render::RenderApp};

use nest_editor_shared::*;

#[no_mangle]
pub extern "C" fn app_builder() -> *mut App {
    let mut app = App::new();
    app.add_plugins(nest_editor_shared::default_plugins_without_windows());
    app.add_plugins(nest_editor_shared::in_game_editor::InGameEditorPlugin);

    app.add_systems(Startup, setup);
    app.add_systems(Update, camera_rotate);

    let entity = app.world_mut().spawn_empty().id();
    let mut e = app.world_mut().entity_mut(entity);
    e.insert(bevy::window::PrimaryWindow);
    e.insert(bevy::window::Window::default());

    app.finish();
    app.cleanup();

    Box::into_raw(Box::new(app))
}

#[no_mangle]
pub unsafe extern "C" fn update_app(app_ptr: *mut App) {
    // Safety: the host must only call this with the pointer returned by create_app,
    // and that pointer must not have been freed yet.
    if let Some(app) = unsafe { app_ptr.as_mut() } {
        // Let Bevy do its update for this frame:
        app.update();
    }
}

pub fn camera_rotate(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Camera3d)>,
) {
    for (mut t, _) in q.iter_mut() {
        t.rotate(Quat::from_rotation_y(ops::sin_cos(time.elapsed_secs() * 2.0 + std::f32::consts::PI / 2.0).0 * 0.002));
    }
}



pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &bevy::window::Window)>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 0))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let window = q.single().0;

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Camera {
        //     // target: bevy::render::camera::RenderTarget::Window(bevy::window::WindowRef::Entity(window)),
        //     ..Default::default()
        // }
    ));
}