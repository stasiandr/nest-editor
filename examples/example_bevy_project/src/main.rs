fn main() {
    let app = example_bevy_project::app_builder();

    // if let Some(app) = unsafe { app.as_mut() } {
    //     // Let Bevy do its update for this frame:
    //     app.run();
    // }
    
    unsafe { example_bevy_project::update_app(app); }
}
