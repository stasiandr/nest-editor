
use libloading::{Library, Symbol};

use crate::user_project;




#[derive(Default)]
pub struct RawAppWrapper {
    app: Option<*mut bevy::app::App>,
    lib: Option<Library>,

    pub window_passed: bool,
}

impl RawAppWrapper {
    pub fn load_from_dylib() -> Self {
        std::process::Command::new("cargo")
            .arg("build")
            .arg("--lib")
            .current_dir(user_project::absolute_project_path())
            .output()
            .expect("Failed to build user project");

        let new_path = user_project::rename_lib_with_random_uuid(user_project::load_dylib_path());

        std::process::Command::new("install_name_tool")
            .arg("-id")
            .arg("''")
            .arg(new_path.clone())
            .output()
            .expect("Install tool failed");

        let lib = unsafe { 
            let filename = new_path;
            Library::new(filename).unwrap() 
        };
        let app_builder: Symbol<extern "C" fn() -> *mut bevy::app::App> = unsafe { lib.get(b"app_builder").unwrap() };
        let app = app_builder();
        Self {
            app: Some(app),
            lib: Some(lib),
            window_passed: false,
        }
    }

    pub fn unload(&mut self) {
        let lib = self.lib.take().unwrap();
        lib.close().unwrap();

        self.app = None;
        self.lib = None;
    }

    pub fn update(&self) {
        let app_builder: Symbol<extern "C" fn(*mut bevy::app::App)> = unsafe { self.lib.as_ref().unwrap().get(b"update_app").unwrap() };
        app_builder(self.app.unwrap());
    }

    pub fn pass_window(&mut self, raw_handle_wrapper: bevy::window::RawHandleWrapper) {
        let set_window_handle_from_app_kit: Symbol<extern "C" fn(app: *mut bevy::app::App, app_kit_handle: *mut std::ffi::c_void)> = unsafe { self.lib.as_ref().unwrap().get(b"set_window_handle_from_app_kit").unwrap() };

        println!("{:?}", raw_handle_wrapper);
        let app_kit_handle = nest_editor_shared::raw_pointer_from_handle_wrapper(raw_handle_wrapper);
        
        set_window_handle_from_app_kit(self.app.unwrap(), app_kit_handle);  
        self.window_passed = true;
    }

    pub fn remove_window(&mut self) {
        let remove_window_handle: Symbol<extern "C" fn(app: *mut bevy::app::App)> = unsafe { self.lib.as_ref().unwrap().get(b"remove_window_handle").unwrap() };
        remove_window_handle(self.app.unwrap());
        self.window_passed = false;
    }

    pub fn is_loaded(&self) -> bool {
        self.app.is_some()
    }
}