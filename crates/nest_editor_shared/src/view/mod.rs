pub mod hierarchy;
pub mod project;

use std::path::PathBuf;

use bevy::prelude::*;
use bevy_egui::egui::{self, include_image, Frame, Ui};
use bevy_inspector_egui::{bevy_inspector::hierarchy::SelectedEntities, dropdown::DropDownBox};
use egui_dock::{DockState, NodeIndex};

use crate::scene_manager::SceneObject;

pub struct NestEditorViewPlugin;

#[derive(Default, Resource)]
pub struct ProjectPath {
    pub path: PathBuf,
}

impl Plugin for NestEditorViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiState::default())
            .add_systems(Update, editor_ui_update)
            .add_systems(Update, set_camera_viewport)
            .insert_resource(ProjectPath {
                path: PathBuf::from("."),
            })
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);
    }

    fn cleanup(&self, _app: &mut App) {
        // egui_logger::builder().init().unwrap();
    }
}

pub fn editor_ui_install(
    world: &mut World
) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy::window::PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };

    let mut ctx = egui_context.clone();
    let _ctx = ctx.get_mut();
}

pub fn editor_ui_update(
    world: &mut World,
) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy::window::PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };


    let mut ctx = egui_context.clone();
    egui_extras::install_image_loaders(ctx.get_mut());

    let mut dock_style = egui_dock::Style::from_egui(&ctx.get_mut().style());

    dock_style.separator.width = 1.0;
    dock_style.tab_bar.corner_radius = egui::CornerRadius::from(16); 
    dock_style.tab_bar.fill_tab_bar = true;
    dock_style.main_surface_border_rounding = egui::CornerRadius::from(16);

    egui::TopBottomPanel::top("top_panel")
        .exact_height(32.0)
        .max_height(32.0)
        .min_height(32.0)
        .frame(Frame::NONE)
        .show(ctx.get_mut(), |ui| {
            let desired_size = egui::Vec2::new(64.0, 16.0);
            let pos = ui.max_rect().center() - desired_size / 2.0;

            ui.allocate_new_ui(
                egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(pos, desired_size)),
                |ui| {
                    let is_in_game_editor = world.get_resource::<crate::in_game_editor::InGameEditorData>().is_some();
                    let button = if is_in_game_editor {
                        egui::Button::image_and_text(include_image!("../../icons/stop.png"), "Stop")
                    } else {
                        egui::Button::image_and_text(include_image!("../../icons/right.png"), "Play")
                    };

                    if ui.add(button).clicked() {
                        if is_in_game_editor {
                            log::info!("Return to editor requested");
                            world.send_event_default::<crate::in_game_editor::ReturnToEditor>();
                        } else {
                            log::info!("Open game requested");
                            world.send_event_default::<crate::in_game_editor::OpenGame>();
                        }
                    }
                },
            );
        });

    egui::CentralPanel::default()
        .frame(Frame::NONE)
        .show(ctx.get_mut(), |ui| {
            world.resource_scope::<UiState, _>(|world, mut ui_state| { 
                ui_state.ui(world, ui, dock_style);
            });
        });
}


fn set_camera_viewport(
    ui_state: Res<UiState>,
    primary_window: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    egui_settings: Query<&bevy_egui::EguiContextSettings>,
    mut cameras: Query<&mut Camera>,
) {
    let cam = cameras.get_single_mut();

    if cam.is_err() {
        return;
    }

    let mut cam = cam.unwrap();

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.single().scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor;

    let physical_position = UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
    let physical_size = UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

    let rect = physical_position + physical_size;

    let window_size = window.physical_size();
    if rect.x <= window_size.x && rect.y <= window_size.y {
        cam.viewport = Some(bevy::render::camera::Viewport {
            physical_position,
            physical_size,
            depth: 0.0..1.0,
        });
    }
}


pub enum WindowType {
    Inspector,
    Viewport,
    World,
    Assets,
    Resources,
    Project,
    Console,
    _Custom(String),
}

impl From<&WindowType> for String {
    fn from(val: &WindowType) -> Self {
        match val {
            WindowType::Inspector => "Inspector".to_string(),
            WindowType::Viewport => "Scene".to_string(),
            WindowType::World => "World".to_string(),
            WindowType::_Custom(name) => name.to_string(),
            WindowType::Assets => "Assets".to_string(),
            WindowType::Resources => "Resources".to_string(),
            WindowType::Project => "Project".to_string(),
            WindowType::Console => "Console".to_string(),
        }
    }
}


#[derive(Resource)]
pub struct UiState {
    state: DockState<WindowType>,
    viewport_rect: egui::Rect,
    selected_entities: SelectedEntities,
    new_component_buffer: String,
}

#[derive(Resource)]
pub struct TabViewer<'a> {
    world: &'a mut World,
    viewport_rect: &'a mut egui::Rect,
    selected_entities: &'a mut SelectedEntities,

    new_component_buffer: &'a mut String,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = WindowType;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        if let WindowType::Viewport = tab {
            if self.world.get_resource::<crate::in_game_editor::InGameEditorData>().is_some() {
                return "Game".into()
            } else {
                return "Scene".into()
            }
        }
        let name :String = (&*tab).into();
        name.into()
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, WindowType::Viewport)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            WindowType::Viewport => {
                *self.viewport_rect = ui.clip_rect();
            }
            WindowType::Inspector => {
                if self.selected_entities.len() == 1 {
                    let entity = self.selected_entities.as_slice()[0];
                    bevy_inspector_egui::bevy_inspector::ui_for_entity(self.world, entity, ui);

                    ui.horizontal(|ui| {
                        self.add_component(ui);
                        if ui.button("add").clicked() {

                            let registry = self.world.resource::<AppTypeRegistry>().clone();
                            let registry = registry.read();

                            let registration = registry
                                .iter()
                                .find(|reg| reg.type_info().type_path() == self.new_component_buffer)
                                .ok_or_else(|| format!("Component {} not found", self.new_component_buffer)).unwrap();
                            

                            if let Some(default) = registration.data::<ReflectDefault>() {
                                let value = default.default();
                                if let Some(registration) = registration.data::<ReflectComponent>() {
                                    let mut entity =  self.world.entity_mut(entity);
                                    
                                    registration.insert(&mut entity, value.as_partial_reflect(), &registry);
                                }
                            }

                            // self.world.commands().entity(entity).insert_by_id(registration, Default::default());
                        }
                    });
                    
                    

                    

                } else {
                    bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(self.world, self.selected_entities.as_slice(), ui);
                }
            }
            WindowType::World => {
                hierarchy::show_hierarchy_ui(self.world, ui, self.selected_entities);
                
                if ui.button("Create entity").clicked() {
                    self.world.commands().spawn(SceneObject);
                }
            }
            WindowType::_Custom(t) => {
                ui.label(format!("Custom tab: {}", t));
            }
            WindowType::Assets => {
                bevy_inspector_egui::bevy_inspector::ui_for_all_assets(self.world, ui);
            },
            WindowType::Resources => {
                bevy_inspector_egui::bevy_inspector::ui_for_resources(self.world, ui);
            }
            WindowType::Project => {
                let project_path = self.world.get_resource_mut::<ProjectPath>().unwrap();
                project::show_project_ui(ui, &project_path.path);
            },
            WindowType::Console => {
                egui_logger::logger_ui().show(ui);
            }
        }
    }


    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        !matches!(tab, WindowType::Viewport)
    }
}

impl TabViewer<'_> {
    pub fn add_component(&mut self, ui: &mut egui::Ui) {
        let registry = self.world.resource::<AppTypeRegistry>();
        let registry = registry.read();

        
        let items = registry.iter()
            .filter(|ty| ty.data::<ReflectComponent>().is_some())
            .map(|ty| ty.type_info().type_path())
            .collect::<Vec<_>>();

        ui.add(DropDownBox::from_iter(
            items, 
            ui.id().with("new_component"), 
            self.new_component_buffer, 
            |ui, path| {
                ui.selectable_label(false, path)
            }
        ));
    }
}

impl Default for UiState {
    fn default() -> Self {
        let mut state = DockState::new(vec![ WindowType::Viewport ]);
        let tree = state.main_surface_mut();
        let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.7, vec![ WindowType::Inspector ]);
        let [world, game] = tree.split_left(game, 0.4, vec![ WindowType::World, WindowType::Assets, WindowType::Resources ]);
        let [_game, _console] = tree.split_below(world, 0.5, vec![ WindowType::Console ]); // TODO fix naming
        let [_world, _project] = tree.split_below(game, 0.7, vec![ WindowType::Project ]);

        Self { 
            state, 
            viewport_rect: egui::Rect::NOTHING,
            selected_entities: SelectedEntities::default(),
            new_component_buffer: String::new(),
        }
    }
}

impl UiState {
    pub fn ui(&mut self, world: &mut World, ui: &mut Ui, style: egui_dock::Style) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            new_component_buffer: &mut self.new_component_buffer,
        };

        egui_dock::DockArea::new(&mut self.state)
            .style(style)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show_inside(ui, &mut tab_viewer);
    }
}
