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

// let (res , _) = self.game_app.as_mut().unwrap().world_mut().query::<(Entity, &bevy::window::RawHandleWrapper)>().single(self.game_app.as_mut().unwrap().world_mut());
                    // self.game_app.as_mut().unwrap().world_mut().entity_mut(res).remove::<bevy::window::RawHandleWrapper>();

//                     let (res , _) = self.editor_app.world_mut().query::<(Entity, &bevy::window::PrimaryWindow)>().single(self.editor_app.world_mut());
//                     self.editor_app.world_mut().entity_mut(res).insert(self.window_handle.as_ref().unwrap().clone());