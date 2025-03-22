use bevy::{app::AppLabel, prelude::*};

#[derive(Debug, AppLabel, Hash, PartialEq, Eq, Clone)]
pub struct BevySubAppLabel;

pub fn standalone() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    // let mut sub_app = SubApp::new();
    // sub_app.add_systems(Startup, setup);
    // app.insert_sub_app(BevySubAppLabel, sub_app);

    app.set_runner(my_runner);
    app.run();
}

fn my_runner(mut app: App) -> AppExit {
    app.finish();
    app.cleanup();

    loop {
        app.update();
        if let Some(exit) = app.should_exit() {
            return exit;
        }
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