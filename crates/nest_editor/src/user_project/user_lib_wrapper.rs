use bevy::app::App;
use libloading::{Library, Symbol};

use super::UserProject;

pub struct UserLibWrapper {
    lib: Library,
}

impl UserLibWrapper {
    pub fn new(user_project: &UserProject) -> Self {
        std::process::Command::new("cargo")
            .arg("build")
            .arg("--lib")
            .current_dir(user_project.absolute_project_path())
            .output()
            .expect("Failed to build user project");

        let lib_path = user_project.copy_lib_to_temp_path();

        let lib = unsafe { Library::new(lib_path).unwrap() };
        
        Self { lib }
    }

    pub fn unload_dylib(self) {
        self.lib.close().unwrap();
    }

    pub fn app_builder(&self) -> *mut App {
        let app_builder: Symbol<extern "C" fn() -> *mut bevy::app::App> = unsafe { self.lib.get(b"app_builder").unwrap() };
        app_builder()
    }

    pub fn update_app(&self, app: *mut App) {
        let update_app: Symbol<extern "C" fn(*mut App)> = unsafe { self.lib.get(b"update_app").unwrap() };
        update_app(app);
    }

    // TODO make macros for those
    pub fn set_window_handle_from_app_kit(&self, app: *mut App, app_kit_handle: *mut std::ffi::c_void) {
        let set_window_handle_from_app_kit: Symbol<extern "C" fn(*mut App, *mut std::ffi::c_void)> = unsafe { self.lib.get(b"set_window_handle_from_app_kit").unwrap() };
        set_window_handle_from_app_kit(app, app_kit_handle);
    }

    pub fn remove_window_handle(&self, app: *mut App) {
        let remove_window_handle: Symbol<extern "C" fn(*mut App)> = unsafe { self.lib.get(b"remove_window_handle").unwrap() };
        remove_window_handle(app);
    }

    pub fn is_back_to_editor_requested(&self, app: *mut App) -> bool {
        let is_back_to_editor_requested: Symbol<extern "C" fn(*mut App) -> bool> = unsafe { self.lib.get(b"is_back_to_editor_requested").unwrap() };
        is_back_to_editor_requested(app)
    }

    pub fn handle_window_resize(&self, app: *mut App, x: u32, y: u32) {
        let handle_window_resize: Symbol<extern "C" fn(*mut App, u32, u32)> = unsafe { self.lib.get(b"handle_window_resize").unwrap() };
        handle_window_resize(app, x, y);
    }

    pub fn handle_mouse_input(&self, app: *mut App, json_serialized: *const i8) {
        let handle_mouse_input: Symbol<extern "C" fn(*mut App, *const i8)> = unsafe { self.lib.get(b"handle_mouse_input").unwrap() };
        handle_mouse_input(app, json_serialized);
    }
    
    pub(crate) fn handle_mouse_move(&self, unwrap: *mut App, x: f64, y: f64) {
        let handle_mouse_move: Symbol<extern "C" fn(*mut App, f64, f64)> = unsafe { self.lib.get(b"handle_mouse_move").unwrap() };
        handle_mouse_move(unwrap, x, y);
    }
    
    pub(crate) fn handle_scale_factor_changed(&self, unwrap: *mut App, scale_factor: f64) {
        let handle_scale_factor_changed: Symbol<extern "C" fn(*mut App, f64)> = unsafe { self.lib.get(b"handle_scale_factor_changed").unwrap() };
        handle_scale_factor_changed(unwrap, scale_factor);
    }
    
    pub(crate) fn handle_mouse_wheel(&self, unwrap: *mut App, x: f64, y: f64, is_line: bool) {
        let handle_mouse_wheel: Symbol<extern "C" fn(*mut App, f64, f64, bool)> = unsafe { self.lib.get(b"handle_mouse_wheel").unwrap() };
        handle_mouse_wheel(unwrap, x, y, is_line);
    }
    
    pub(crate) fn handle_keyboard_event(&self, app: *mut App, json_serialized: *const i8) {
        let handle_keyboard_event: Symbol<extern "C" fn(*mut App, *const i8)> = unsafe { self.lib.get(b"handle_keyboard_event").unwrap() };
        handle_keyboard_event(app, json_serialized);
    }
}
