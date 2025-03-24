use std::path::PathBuf;
use bevy::prelude::*;
use uuid::Uuid;



pub fn load_app_builder_from_dylib(lib_path: PathBuf) -> App {
    unsafe {
        let lib = libloading::Library::new(lib_path).unwrap();
        let app_builder: libloading::Symbol<unsafe extern fn() -> App> = lib.get(b"app_builder").unwrap();
        app_builder()
    }
}


pub fn load_dylib_path() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    let default_test_project_path = "examples/example_bevy_project".to_string();
    let project_path = args.get(1).unwrap_or(&default_test_project_path);

    let absolute_project_path = std::fs::canonicalize(project_path).unwrap();
    let project_name = absolute_project_path.file_name().unwrap().to_str().unwrap();

    absolute_project_path.join("target/debug/").join(format!("lib{project_name}.dylib"))
}

pub fn rename_lib_with_random_uuid(path: PathBuf) -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    let default_test_project_path = "examples/example_bevy_project".to_string();
    let project_path = args.get(1).unwrap_or(&default_test_project_path);

    let absolute_project_path = std::fs::canonicalize(project_path).unwrap();
    let project_name = absolute_project_path.file_name().unwrap().to_str().unwrap();

    let uuid = Uuid::new_v4();
    let new_path = absolute_project_path.join("target/debug/").join(format!("lib{project_name}_{uuid}.dylib"));
    std::fs::rename(&path, &new_path).unwrap();
    new_path
}

pub fn absolute_project_path() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    let default_test_project_path = "examples/example_bevy_project".to_string();
    let project_path = args.get(1).unwrap_or(&default_test_project_path);

    std::fs::canonicalize(project_path).unwrap()
}