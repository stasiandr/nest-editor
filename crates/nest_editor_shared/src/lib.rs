pub mod in_game_editor;

use bevy::prelude::PluginGroup;
use in_game_editor::ReturnToEditor;
use std::ptr::NonNull;

use bevy::{app::Plugin, ecs::entity::Entity, window::RawHandleWrapper};
use winit::raw_window_handle::{AppKitDisplayHandle, RawWindowHandle};

pub struct NestEditorSharedPlugin;

impl Plugin for NestEditorSharedPlugin {
    fn build(&self, _app: &mut bevy::app::App) {
        println!("Nest editor is installed!")
    }
}

#[no_mangle]
pub extern "C" fn test() {
    println!("Hello from nest_editor_shared!");
}


pub fn handle_wrapper_from_app_kit(app_kit_handle: *mut std::ffi::c_void) -> RawHandleWrapper {
    let ns_view = app_kit_handle;
    let app_kit_handle = winit::raw_window_handle::AppKitWindowHandle::new(NonNull::new(ns_view).unwrap());
    let window_handle = RawWindowHandle::AppKit(app_kit_handle);

    let display_handle = winit::raw_window_handle::RawDisplayHandle::AppKit(AppKitDisplayHandle::new());

    bevy::window::RawHandleWrapper::from_handles(window_handle, display_handle).unwrap()
}

pub fn raw_pointer_from_handle_wrapper(handle: RawHandleWrapper) -> *mut std::ffi::c_void {
    let window_handle = handle.window_handle;
    let app_kit_handle = match window_handle {
        RawWindowHandle::AppKit(app_kit_handle) => app_kit_handle,
        _ => panic!("Not an AppKit handle"),
    };

    app_kit_handle.ns_view.as_ptr()
}



/// # Safety
/// I'm not in danger, I'm the danger
#[no_mangle]
pub unsafe extern "C" fn set_window_handle_from_app_kit(app: *mut bevy::app::App, app_kit_handle: *mut std::ffi::c_void) {
    let app = app.as_mut().unwrap();
    let wrapper = handle_wrapper_from_app_kit(app_kit_handle);    
    let (res, _)  = app.world_mut().query::<(Entity, &bevy::window::PrimaryWindow)>().single(app.world_mut());
    app.world_mut().entity_mut(res).insert(wrapper);
}

/// # Safety
/// I'm not in danger, I'm the danger
#[no_mangle]
pub unsafe extern "C" fn remove_window_handle(app: *mut bevy::app::App) {
    let app = unsafe { app.as_mut().unwrap() };
    let (res , _) = app.world_mut().query::<(bevy::ecs::entity::Entity, &bevy::window::RawHandleWrapper)>().single(app.world_mut());
    app.world_mut().entity_mut(res).remove::<bevy::window::RawHandleWrapper>();
}

pub fn default_plugins_without_windows() -> bevy::app::PluginGroupBuilder {
    bevy::DefaultPlugins.build()
        // .disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>()
        .disable::<bevy::winit::WinitPlugin>()
        .set(bevy::window::WindowPlugin {
            primary_window: None,
            exit_condition: bevy::window::ExitCondition::DontExit,
            ..Default::default()
        })
}

/// # Safety
/// I'm not in danger, I'm the danger
#[no_mangle]
pub unsafe extern "C" fn is_back_to_editor_requested(app: *mut bevy::app::App) -> bool {
    let app = unsafe { app.as_mut().unwrap() };
    let events = app.world().resource::<bevy::ecs::event::Events<ReturnToEditor>>();
    !events.is_empty()
}

/// # Safety
/// I'm not in danger, I'm the danger
#[no_mangle]
pub unsafe extern "C" fn handle_window_resize(app: *mut bevy::app::App, x: u32, y: u32) {
    let app = unsafe { app.as_mut().unwrap() };
    let mut win = app.world_mut().query::<&mut bevy::window::Window>().single_mut(app.world_mut());
    win.resolution.set_physical_resolution(x, y);
}

