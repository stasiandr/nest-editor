use std::{collections::BTreeMap, path::{Path, PathBuf}};

use bevy::prelude::*;
use bevy_egui::egui::{self, include_image, Frame, Ui};
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use egui_dock::{DockState, NodeIndex};
use egui_ltreeview::{TreeView, TreeViewBuilder};
use ignore::WalkBuilder;

#[derive(Default)]
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
        egui_logger::builder().init().unwrap();
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
    // dock_style.tab.inactive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 0);

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
                        egui::Button::image_and_text(include_image!("../icons/stop.png"), "Stop")
                    } else {
                        egui::Button::image_and_text(include_image!("../icons/right.png"), "Play")
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
    let mut cam = cameras.single_mut();

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
}

#[derive(Resource)]
pub struct TabViewer<'a> {
    world: &'a mut World,
    viewport_rect: &'a mut egui::Rect,
    selected_entities: &'a mut SelectedEntities,
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
                } else {
                    bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(self.world, self.selected_entities.as_slice(), ui);
                }
            }
            WindowType::World => {
                bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(self.world, ui, self.selected_entities);
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
                show_project_ui(ui, &project_path.path);
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


#[derive(Debug, Clone)]
enum FileNode {
    Directory(BTreeMap<String, FileNode>),
    File,
}


fn build_tree(root: &Path) -> FileNode {
    let mut root_map = BTreeMap::new();

    for result in WalkBuilder::new(root)
        .standard_filters(true)
        .follow_links(false)
        .build()
    {
        let Ok(entry) = result else { continue; };
        let path = entry.path();

        if path == root {
            continue;
        }

        let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());

        let Ok(rel_path) = path.strip_prefix(root) else { continue; };
        if rel_path.as_os_str().is_empty() {
            continue; // Shouldn't happen, but be safe
        }

        insert_into_tree(&mut root_map, rel_path, is_dir);
    }

    FileNode::Directory(root_map)
}

fn insert_into_tree(tree: &mut BTreeMap<String, FileNode>, rel_path: &Path, is_dir: bool) {
    if let Some(first) = rel_path.iter().next() {
        let key = first.to_string_lossy().to_string();
        let remainder = rel_path.strip_prefix(first).unwrap_or(rel_path);
        let entry = tree.entry(key).or_insert_with(|| {
            if is_dir {
                FileNode::Directory(BTreeMap::new())
            } else {
                FileNode::File
            }
        });

        if remainder.components().next().is_none() {
            if is_dir {
                *entry = FileNode::Directory(
                    match entry {
                        FileNode::Directory(ref m) => m.clone(),
                        FileNode::File => BTreeMap::new(),
                    }
                );
            } else {
                *entry = FileNode::File;
            }
        } else if let FileNode::Directory(ref mut subtree) = entry {
            insert_into_tree(subtree, remainder, is_dir);
        } else {
            let mut new_map = BTreeMap::new();
            insert_into_tree(&mut new_map, remainder, is_dir);
            *entry = FileNode::Directory(new_map);
        }
    }
}

fn show_nested_tree(ui: &mut egui::Ui, root: &std::path::Path) {
    let file_node = build_tree(root);
    if let FileNode::Directory(tree_map) = file_node {
        TreeView::new("nested_tree".into()).show(ui, |builder| {
            let mut next_id = 0_usize;
            let root_label = root.to_string_lossy().to_string();
            
            render_tree(builder, &tree_map, &mut next_id, &root_label);
        });
    }
}

fn render_tree(
    builder: &mut TreeViewBuilder<usize>,
    nodes: &std::collections::BTreeMap<String, FileNode>,
    next_id: &mut usize,
    label: &str,
) {
    let this_dir_id = *next_id;
    *next_id += 1;

    builder.dir(this_dir_id, label);
    for (name, node) in nodes {
        match node {
            FileNode::File => {
                let leaf_id = *next_id;
                *next_id += 1;

                builder.leaf(leaf_id, name);
            }
            FileNode::Directory(subtree) => {
                render_tree(builder, subtree, next_id, name);
            }
        }
    }

    builder.close_dir();
}


fn show_project_ui(ui: &mut Ui, project_path: &Path) {
    show_nested_tree(ui, project_path);
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
        }
    }
}

impl UiState {
    pub fn ui(&mut self, world: &mut World, ui: &mut Ui, style: egui_dock::Style) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
        };

        egui_dock::DockArea::new(&mut self.state)
            .style(style)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show_inside(ui, &mut tab_viewer);
    }
}
