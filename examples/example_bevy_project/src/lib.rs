use bevy::prelude::*;


#[no_mangle]
pub fn app_builder() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.build()
        .disable::<bevy::winit::WinitPlugin>()
        .set(WindowPlugin {
            primary_window: None,
            exit_condition: bevy::window::ExitCondition::DontExit,
            ..Default::default()
        })
    );

    app.add_systems(Startup, setup);
    app.add_systems(Update, camera_rotate);

    app
}


fn camera_rotate(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Camera3d)>,
) {
    for (mut t, _) in q.iter_mut() {
        t.rotate(Quat::from_rotation_y(ops::sin_cos(time.delta_secs()).0 * 0.1));
    }

}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("startup worked");
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
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
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    println!("I'm alive")
}