use bevy::ecs::{query::With, reflect::AppTypeRegistry, world::World};
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;

use crate::scene_manager::SceneObject;


pub fn show_hierarchy_ui(world: &mut World, ui: &mut egui::Ui, selected: &mut SelectedEntities) -> bool {
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let type_registry = type_registry.read();

    bevy_inspector_egui::bevy_inspector::hierarchy::Hierarchy {
        world,
        type_registry: &type_registry,
        selected,
        context_menu: None,
        shortcircuit_entity: None,
        extra_state: &mut (),
    }
    .show::<With<SceneObject>>(ui)
}
