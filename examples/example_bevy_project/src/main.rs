fn main() {
    let app = example_bevy_project::app_builder();
    let app = unsafe { app.as_mut().unwrap() };
    app.finish();
    app.cleanup();

    println!("{:?}", app.world().get_resource::<bevy::ecs::schedule::Schedules>().is_some());
    app.update();
}
