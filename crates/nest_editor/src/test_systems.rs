use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasing, prelude::*};

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
    )).with_child((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    
    // light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 1000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            std::f32::consts::PI * 0.15,
            std::f32::consts::PI * -0.15,
        )),
    ));

    let window = q.single().0;

    // camera
    commands.spawn((
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
    ));
}