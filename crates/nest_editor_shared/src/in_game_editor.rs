use bevy::prelude::*;


#[derive(Event, Default)]
pub struct ReturnToEditor;

#[derive(Event, Default)]
pub struct OpenGame;

#[derive(Resource, Default)]
pub struct InGameEditorData;

pub struct InGameEditorPlugin;

impl Plugin for InGameEditorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugins(bevy_egui::EguiPlugin)
            .add_event::<ReturnToEditor>()
            .add_plugins(crate::view::NestEditorViewPlugin)
            .init_resource::<InGameEditorData>();
    }
}