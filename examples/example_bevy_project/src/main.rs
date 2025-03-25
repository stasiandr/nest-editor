fn main() {
    let app = example_bevy_project::app_builder();

    if let Some(app) = unsafe { app.as_mut() } {
    
        loop {
            app.update();
        }

    }
}
