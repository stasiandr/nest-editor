use bevy::prelude::*;
use bevy_egui::{egui::{self, Widget}, EguiContexts};
use egui_dock::DockState;

#[derive(Default)]
pub struct NestEditorViewPlugin;

impl Plugin for NestEditorViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NestEditorState::default());
        app.insert_resource(TabViewer::default());
        app.add_systems(Update, editor_ui_update);
    }
}


pub fn editor_ui_update(
    mut contexts: EguiContexts,
    mut _app_state: ResMut<NestEditorState>,
    mut _tab_viewer: ResMut<TabViewer>,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("title").show(ctx, |ui| {
        egui::Button::new("Run game").ui(ui);
    });
    // let tab_viewer = tab_viewer.as_mut();
    // egui_dock::DockArea::new(&mut app_state.tree)
    //     .show(contexts.ctx_mut(), tab_viewer);
}


pub enum WindowType {
    Inspector,
    Game,
    World,
    _Custom(String),
}

impl From<&WindowType> for String {
    fn from(val: &WindowType) -> Self {
        match val {
            WindowType::Inspector => "Inspector".to_string(),
            WindowType::Game => "Game".to_string(),
            WindowType::World => "World".to_string(),
            WindowType::_Custom(name) => name.to_string(),
        }
    }
}


#[derive(Resource)]
pub struct NestEditorState {
    pub tree: DockState<WindowType>
}

#[derive(Default, Resource)]
pub struct TabViewer {

}

impl egui_dock::TabViewer for TabViewer {
    type Tab = WindowType;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let name :String = (&*tab).into();
        name.into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let name :String = (&*tab).into();
        ui.label(format!("Content of {name}"));
    }
}

impl Default for NestEditorState {
    fn default() -> Self {
        let tree = DockState::new(vec![
            WindowType::World, 
            WindowType::Game, 
            WindowType::Inspector
        ]);

        Self { tree }
    }
}

