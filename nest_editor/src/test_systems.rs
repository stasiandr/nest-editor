use std::f32::consts::PI;

use bevy::prelude::*;

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

    let window = q.single().0;

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            target: bevy::render::camera::RenderTarget::Window(bevy::window::WindowRef::Entity(window)),
            ..Default::default()
        }
    ));
}


pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
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
}

pub fn camera_rotate(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Camera3d)>,
) {
    println!("{}", ops::sin_cos(time.elapsed_secs()).0);

    for (mut t, _) in q.iter_mut() {
        t.rotate(Quat::from_rotation_y(ops::sin_cos(time.elapsed_secs() + PI / 2.0).0 * 0.001));
    }
}


pub fn configure_context(mut egui_settings: Query<&mut bevy_egui::EguiContextSettings>) {
    for mut es in egui_settings.iter_mut() {
        es.run_manually = true;
    }
}

pub fn ui_example_system(
    mut contexts: Query<(&mut bevy_egui::EguiContext, &mut bevy_egui::EguiInput, &mut bevy_egui::EguiFullOutput)>
) {
    let (mut ctx, mut egui_input, mut egui_full_output) = contexts.single_mut();

    let ui = |ctx: &bevy_egui::egui::Context| {
        bevy_egui::egui::Window::new("Hello").show(ctx, |ui| {
            
            if (ui.button("Run game")).clicked() {
                println!("Button clicked!");
            }
        });
    };

    let ctx = ctx.get_mut();

    **egui_full_output = Some(ctx.run(egui_input.take(), ui));
}