fn main() {
    let app = example_bevy_project::app_builder();
    unsafe { example_bevy_project::update_app(app); }
}
