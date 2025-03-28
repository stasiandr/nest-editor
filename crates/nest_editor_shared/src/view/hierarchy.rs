use bevy::ecs::{query::With, reflect::AppTypeRegistry, world::World};
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;

use crate::scene_manager::SceneObject;


pub fn show_hierarchy_ui(world: &mut World, ui: &mut egui::Ui, selected: &mut SelectedEntities) -> bool {
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let type_registry = type_registry.read();
    let is_in_game_editor = world.get_resource::<crate::in_game_editor::InGameEditorData>().is_some();

    let mut hierarchy = bevy_inspector_egui::bevy_inspector::hierarchy::Hierarchy {
        world,
        type_registry: &type_registry,
        selected,
        context_menu: None,
        shortcircuit_entity: None,
        extra_state: &mut (),
    };
    
    if !is_in_game_editor {
        hierarchy.show::<With<SceneObject>>(ui)
    } else {
        hierarchy.show::<()>(ui)
    }
    
}
