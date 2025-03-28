use bevy::{log::LogPlugin, prelude::*};

use nest_editor_shared::*;

#[no_mangle]
pub extern "C" fn app_builder() -> *mut App {
    let mut app = App::new();
    app.add_plugins(nest_editor_shared::default_plugins_without_windows("/Users/stas/learn/nest-editor/examples/example_bevy_project/assets".to_string()));
    app.add_plugins(nest_editor_shared::in_game_editor::InGameEditorPlugin);

    app.add_systems(Startup, setup);

    let entity = app.world_mut().spawn_empty().id();
    let mut e = app.world_mut().entity_mut(entity);
    e.insert(bevy::window::PrimaryWindow);
    e.insert(bevy::window::Window::default());

    app.finish();
    app.cleanup();

    Box::into_raw(Box::new(app))
}

pub fn app() -> App {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
    app.add_plugins(nest_editor_shared::in_game_editor::InGameEditorPlugin);
    app.add_systems(Startup, setup);
    app
}

/// # Safety
/// No nothing about
#[no_mangle]
pub unsafe extern "C" fn update_app(app_ptr: *mut App) {
    // Safety: the host must only call this with the pointer returned by create_app,
    // and that pointer must not have been freed yet.
    if let Some(app) = unsafe { app_ptr.as_mut() } {
        // Let Bevy do its update for this frame:
        app.update();
    }
}

pub fn setup(
    mut commands: Commands,
    q: Query<Entity, With<bevy::window::Window>>,
) {
    if let Ok(window) = q.get_single() {
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
}