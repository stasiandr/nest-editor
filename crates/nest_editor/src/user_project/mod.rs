use std::path::PathBuf;
use bevy::prelude::*;



pub fn load_app_builder_from_dylib(lib_path: PathBuf) -> App {
    unsafe {
        let lib = libloading::Library::new(lib_path).unwrap();
        let app_builder: libloading::Symbol<unsafe extern fn() -> App> = lib.get(b"app_builder").unwrap();
        app_builder()
    }
}


pub fn load_dylib_path() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    let project_path = args.get(1);

    if project_path.is_none() {
        panic!("Please provide a project path");
    }
    let project_path = project_path.unwrap();
    let absolute_project_path = std::fs::canonicalize(project_path).unwrap();
    let project_name = absolute_project_path.file_name().unwrap().to_str().unwrap();

    absolute_project_path.join("target/debug/").join(format!("lib{project_name}.dylib"))
}