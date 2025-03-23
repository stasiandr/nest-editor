use crate::user_project;




pub struct RawAppWrapper {
    app: *mut bevy::app::App,
    lib: libloading::Library,
}

impl RawAppWrapper {
    pub fn load_from_dylib() -> Self {
        let lib = unsafe { libloading::Library::new(user_project::load_dylib_path()).unwrap() };
        let app_builder: libloading::Symbol<extern "C" fn() -> *mut bevy::app::App> = unsafe { lib.get(b"app_builder").unwrap() };
        let app = app_builder();
        Self {
            app,
            lib,
        }
    }

    pub fn update(&self) {
        let app_builder: libloading::Symbol<extern "C" fn(*mut bevy::app::App)> = unsafe { self.lib.get(b"update_app").unwrap() };
        app_builder(self.app);
    }

    pub fn pass_window(&self, raw_handle_wrapper: bevy::window::RawHandleWrapper) {
        let set_window_handle_from_app_kit: libloading::Symbol<extern "C" fn(app: *mut bevy::app::App, app_kit_handle: *mut std::ffi::c_void)> = unsafe { self.lib.get(b"set_window_handle_from_app_kit").unwrap() };

        let app_kit_handle = nest_editor_shared::raw_pointer_from_handle_wrapper(raw_handle_wrapper);
        set_window_handle_from_app_kit(self.app, app_kit_handle);  
    }

    pub fn remove_window(&self) {
        let remove_window_handle: libloading::Symbol<extern "C" fn(app: *mut bevy::app::App)> = unsafe { self.lib.get(b"remove_window_handle").unwrap() };
        remove_window_handle(self.app);
    }
}