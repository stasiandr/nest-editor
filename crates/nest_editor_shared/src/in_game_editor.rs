use bevy::prelude::*;


#[derive(Event)]
pub struct ReturnToEditor;

#[derive(Event, Default)]
pub struct OpenGame;

pub struct InGameEditorPlugin;

impl Plugin for InGameEditorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(bevy_egui::EguiPlugin);
        app.add_event::<ReturnToEditor>();
        app.add_systems(
            PreStartup,
            configure_context.after(bevy_egui::EguiStartupSet::InitContexts),
        )
        .add_systems(Update, ui_example_system);
    }
}


pub fn configure_context(mut egui_settings: Query<&mut bevy_egui::EguiContextSettings>) {
    for mut es in egui_settings.iter_mut() {
        es.run_manually = true;
    }
}

pub fn ui_example_system(
    mut switch_context_event: EventWriter<ReturnToEditor>,
    mut contexts: Query<(&mut bevy_egui::EguiContext, &mut bevy_egui::EguiInput, &mut bevy_egui::EguiFullOutput)>
) {
    let (mut ctx, mut egui_input, mut egui_full_output) = contexts.single_mut();

    let ui = |ctx: &bevy_egui::egui::Context| {
        bevy_egui::egui::Window::new("Hello").show(ctx, |ui| {
            if (ui.button("Back to Editor")).clicked() {
                switch_context_event.send(ReturnToEditor);
                println!("Button clicked!");
            }
        });
    };

    let ctx = ctx.get_mut();

    **egui_full_output = Some(ctx.run(egui_input.take(), ui));
}