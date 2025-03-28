use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::{scene_manager::SharedSceneManager, view::NestEditorViewPlugin};


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
            .add_event::<ReturnToEditor>()
            .init_resource::<InGameEditorData>()
            .add_plugins(EguiPlugin)
            .add_plugins(NestEditorViewPlugin)
            .add_plugins(SharedSceneManager);
    }
}