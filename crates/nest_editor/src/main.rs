pub mod user_project;
pub mod utils;
pub mod editor_camera;
pub mod editor_app_utils;



use bevy::prelude::*;
use bevy::app::App;
use nest_editor_shared::in_game_editor::OpenGame;
use user_project::user_app::{UserApp, UserAppState};
use winit::{event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};


pub struct EditorApp {
    editor_app: App,
    user_project: user_project::UserProject,
    main_window: MainWindow,
    game_app: UserApp,

    pub windows: bevy::utils::HashMap<WindowId, bevy::window::WindowWrapper<Window>>,
}


#[derive(Default)]
struct MainWindow {
    editor_window_entity: Option<Entity>,
    window_id: Option<WindowId>,
    window_handle: Option<bevy::window::RawHandleWrapper>,
}

impl winit::application::ApplicationHandler for EditorApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        self.insert_window_into_editor_app(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {

        match event {
            WindowEvent::MouseWheel { delta, .. } => {
                if !self.game_app.get_app_state().is(UserAppState::WindowPassedToGame) {
                    self.handle_mouse_wheel(delta);
                } else {
                    self.game_app.handle_mouse_wheel(delta);
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                if !self.game_app.get_app_state().is(UserAppState::WindowPassedToGame) {
                    self.handle_scale_factor_changed(scale_factor);
                } else {
                    self.game_app.handle_scale_factor_changed(scale_factor);
                }
            }
            WindowEvent::Resized(size) => {
                if !self.game_app.get_app_state().is(UserAppState::WindowPassedToGame) {
                    self.handle_window_resize(size);
                } else {
                    self.game_app.handle_window_resize(size);
                }
            }

            WindowEvent::CursorMoved { device_id: _, position } => {
                if !self.game_app.get_app_state().is(UserAppState::WindowPassedToGame) {
                    self.handle_mouse_move(position);
                } else {
                    self.game_app.handle_mouse_move(position);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let event = bevy::input::mouse::MouseButtonInput {
                    button: utils::convert_mouse_button(button),
                    state: utils::convert_element_state(state),
                    window: self.main_window.editor_window_entity.unwrap(),
                };    

                if !self.game_app.get_app_state().at_least(UserAppState::WindowPassedToGame) {
                    self.editor_app.world_mut().send_event(event);
                } else {
                    self.game_app.handle_mouse_input(&event);
                }
            }
            WindowEvent::KeyboardInput { ref event, .. } => {
                let event = utils::convert_keyboard_input(event, self.main_window.editor_window_entity.unwrap());
                if !self.game_app.get_app_state().is(UserAppState::WindowPassedToGame) {
                    self.editor_app.world_mut().send_event(event);
                } else {
                    self.game_app.handle_keyboard_event(&event);
                }
            }
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if self.game_app.get_app_state().at_least(UserAppState::WindowPassedToGame) {
                    self.game_app.update_app();
                } else {
                    self.editor_app.update();
                }

                if self.game_app.is_back_to_editor_requested() {
                    self.game_app.remove_window();
                    self.insert_raw_handle_wrapper();
                    self.game_app.kill_app();
                    self.game_app.unload_lib();

                    let mut win_entity = self.editor_app.world_mut().entity_mut(self.main_window.editor_window_entity.unwrap());
                    let win = win_entity.get_mut::<bevy::window::Window>().unwrap();
                    let size = win.resolution.physical_size();
                    let size = winit::dpi::PhysicalSize::new(size.x, size.y);
                    let scale_factor = win.resolution.scale_factor().into();
                    self.handle_scale_factor_changed(scale_factor);
                    self.handle_window_resize(size);
                }

                let mut events = self.editor_app.world_mut().resource_mut::<bevy::ecs::event::Events<nest_editor_shared::in_game_editor::OpenGame>>();
                if events.drain().count() != 0 {
                    println!("Open game requested");
                    self.game_app.load_lib(&self.user_project);
                    self.game_app.build_app();

                    self.remove_raw_handle_wrapper();

                    self.game_app.pass_window(self.main_window.window_handle.clone().unwrap());

                    let mut win_entity = self.editor_app.world_mut().entity_mut(self.main_window.editor_window_entity.unwrap());
                    let win = win_entity.get_mut::<bevy::window::Window>().unwrap();
                    let size = win.resolution.physical_size();
                    let size = winit::dpi::PhysicalSize::new(size.x, size.y);
                    self.game_app.handle_window_resize(size);
                    self.game_app.handle_scale_factor_changed(win.resolution.scale_factor().into());
                }

                for w in self.windows.values() {
                    w.request_redraw();
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let user_project = user_project::UserProject::new("examples/example_bevy_project".into());

    let mut editor_app = App::new();
    

    editor_app.add_plugins(nest_editor_shared::default_plugins_without_windows(user_project.project_assets_path().to_str().unwrap().to_string()));
    editor_app.add_plugins(bevy_egui::EguiPlugin);
    
    editor_app.add_systems(Startup, editor_camera::spawn_editor_camera);
    editor_app.add_systems(Update, editor_camera::editor_camera_controls);
    editor_app.insert_resource(AmbientLight {
        brightness: 50.0,
        color: Color::WHITE,
    });
    editor_app.add_event::<OpenGame>();
    editor_app.add_plugins(nest_editor_shared::scene_manager::EditorSceneManager);
    editor_app.add_plugins(nest_editor_shared::view::NestEditorViewPlugin);

    
    editor_app.insert_resource(user_project.clone());

    let mut winit_app = EditorApp {
        editor_app,
        user_project,
        main_window: MainWindow::default(),
        game_app: UserApp::default(),
        windows: bevy::utils::HashMap::default(),
    };
    event_loop.run_app(&mut winit_app).unwrap();
}