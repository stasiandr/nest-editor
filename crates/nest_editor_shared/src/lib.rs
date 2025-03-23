use bevy::app::Plugin;

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