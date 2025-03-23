use std::ptr::NonNull;

use bevy::app::Plugin;
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

#[no_mangle]
pub extern "C" fn set_raw_handle_wrapper(app_kit_handle: *mut std::ffi::c_void) {
    
    let ns_view = app_kit_handle;
    let app_kit_handle = winit::raw_window_handle::AppKitWindowHandle::new(NonNull::new(ns_view).unwrap());
    let window_handle = RawWindowHandle::AppKit(app_kit_handle);

    let display_handle = winit::raw_window_handle::RawDisplayHandle::AppKit(AppKitDisplayHandle::new());

    let _wrapper = bevy::window::RawHandleWrapper::from_handles(window_handle, display_handle);
}

// let (res , _) = self.game_app.as_mut().unwrap().world_mut().query::<(Entity, &bevy::window::RawHandleWrapper)>().single(self.game_app.as_mut().unwrap().world_mut());
                    // self.game_app.as_mut().unwrap().world_mut().entity_mut(res).remove::<bevy::window::RawHandleWrapper>();

//                     let (res , _) = self.editor_app.world_mut().query::<(Entity, &bevy::window::PrimaryWindow)>().single(self.editor_app.world_mut());
//                     self.editor_app.world_mut().entity_mut(res).insert(self.window_handle.as_ref().unwrap().clone());