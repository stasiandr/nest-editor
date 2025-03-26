use bevy::prelude::*;
use bevy_egui::egui::{self, CornerRadius};
use egui_dock::{DockState, NodeIndex};

#[derive(Default)]
pub struct NestEditorViewPlugin;

impl Plugin for NestEditorViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiState::default())
            .add_systems(Update, editor_ui_update)
            .add_systems(Update, set_camera_viewport);
    }
}

pub fn editor_ui_update(
    world: &mut World
) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy::window::PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
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

    // The desired viewport rectangle at its offset in "physical pixel space"
    let rect = physical_position + physical_size;

    let window_size = window.physical_size();
    // wgpu will panic if trying to set a viewport rect which has coordinates extending
    // past the size of the render target, i.e. the physical window in our case.
    // Typically this shouldn't happen- but during init and resizing etc. edge cases might occur.
    // Simply do nothing in those cases.
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
    _Custom(String),
}

impl From<&WindowType> for String {
    fn from(val: &WindowType) -> Self {
        match val {
            WindowType::Inspector => "Inspector".to_string(),
            WindowType::Viewport => "Scene".to_string(),
            WindowType::World => "World".to_string(),
            WindowType::_Custom(name) => name.to_string(),
        }
    }
}


#[derive(Resource)]
pub struct UiState {
    state: DockState<WindowType>,
    viewport_rect: egui::Rect,
}

#[derive(Resource)]
pub struct TabViewer<'a> {
    _world: &'a mut World,
    viewport_rect: &'a mut egui::Rect,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = WindowType;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let name :String = (&*tab).into();
        name.into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            WindowType::Viewport => {
                *self.viewport_rect = ui.clip_rect();
            }
            WindowType::Inspector => {
                ui.label("Inspector");
            }
            WindowType::World => {
                ui.label("World");
            }
            WindowType::_Custom(t) => {
                ui.label(format!("Custom tab: {}", t));
            }
        }
    }


    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        !matches!(tab, WindowType::Viewport)
    }
}

impl Default for UiState {
    fn default() -> Self {
        let mut state = DockState::new(vec![ WindowType::Viewport ]);
        let tree = state.main_surface_mut();
        let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.7, vec![ WindowType::Inspector ]);
        let [_world, _game] = tree.split_left(game, 0.3, vec![ WindowType::World ]);
        
        let game_node = tree.iter_mut().find(|node| { // TODO replace with node-index
            if let Some(tabs) = node.tabs() {
                if tabs.len() == 1 && matches!(tabs[0], WindowType::Viewport) {
                    return true;
                }
            }
            false
        });

        game_node.unwrap().set_collapsed(true);

        Self { 
            state, 
            viewport_rect: egui::Rect::NOTHING
        }
    }
}

impl UiState {
    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            _world: world,
            viewport_rect: &mut self.viewport_rect,
        };

        let game_node = self.state.main_surface_mut().iter_mut().find(|node| { // TODO replace with node-index
            if let Some(tabs) = node.tabs() {
                if tabs.len() == 1 && matches!(tabs[0], WindowType::Viewport) {
                    return true;
                }
            }
            false
        });
        
        if let Some(node) = game_node {
            node.set_collapsed(true);
        }

        

        let mut style = egui_dock::Style::from_egui(&ctx.style());

        style.separator.width = 1.0;
        style.tab_bar.corner_radius = CornerRadius::from(8); 
        style.tab_bar.fill_tab_bar = true;

        egui_dock::DockArea::new(&mut self.state)
            .style(style)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(ctx, &mut tab_viewer);
    }
}
