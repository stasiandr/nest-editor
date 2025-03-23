pub mod view;
pub mod user_project;
pub mod utils;
pub mod test_systems;


use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowWrapper};
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};
use winit::{event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};


#[derive(Default)]
struct WinitApp {
    editor_app: App,
    main_window: MainWindow,
}

#[derive(Default)]
struct MainWindow {
    editor_window_entity: Option<Entity>,
    window_id: Option<WindowId>,
    window_handle: Option<bevy::window::RawHandleWrapper>,
}

impl winit::application::ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        let editor_window_id = window.id();
    
        let mut windows = bevy_winit::WinitWindows::default();
        let entity = self.editor_app.world_mut().spawn_empty().id();
        windows.entity_to_winit.insert(entity, editor_window_id);
        windows.winit_to_entity.insert(editor_window_id, entity);
        
        let wrapper = windows.windows
            .entry(editor_window_id)
            .insert(WindowWrapper::new(window))
            .into_mut();

        let mut e = self.editor_app.world_mut().entity_mut(entity);
        e.insert(PrimaryWindow);
        e.insert(bevy::window::Window::default());
        e.insert(bevy::window::RawHandleWrapper::new(wrapper).unwrap());

        self.main_window.window_handle = Some(bevy::window::RawHandleWrapper::new(wrapper).unwrap());

        self.editor_app.insert_non_send_resource(windows);
        self.editor_app.finish();
        self.editor_app.cleanup();
        self.main_window.editor_window_entity = Some(entity);

        self.main_window.window_id = Some(editor_window_id);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        
        let mut win_entity = self.editor_app.world_mut().entity_mut(self.main_window.editor_window_entity.unwrap());
        let mut win = win_entity.get_mut::<bevy::window::Window>().unwrap();
        
        match event {
            WindowEvent::Resized(size) => {
                win.resolution.set_physical_resolution(size.width, size.height);
            }

            WindowEvent::CursorMoved { device_id: _, position } => {
                let physical_position = bevy::math::DVec2::new(position.x, position.y);

                let last_position = win.physical_cursor_position();
                let delta = last_position.map(|last_pos| {
                    (physical_position.as_vec2() - last_pos) / win.resolution.scale_factor()
                });

                win.set_physical_cursor_position(Some(physical_position));
                let position = (physical_position / win.resolution.scale_factor() as f64).as_vec2();
                let event = CursorMoved {
                    window: self.main_window.editor_window_entity.unwrap(),
                    position,
                    delta,
                };
                self.editor_app.world_mut().send_event(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.editor_app.world_mut().send_event(bevy::input::mouse::MouseButtonInput {
                    button: utils::convert_mouse_button(button),
                    state: utils::convert_element_state(state),
                    window: self.main_window.editor_window_entity.unwrap(),
                });
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    return;
                }

                println!("Switching targets");
            }
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                
                self.editor_app.update();

                let window = self.editor_app.world().non_send_resource::<bevy_winit::WinitWindows>().get_window(self.main_window.editor_window_entity.unwrap());
                window.unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut editor_app = App::new();

    editor_app.add_plugins(DefaultPlugins.build()
        .disable::<bevy::winit::WinitPlugin>()
        .set(WindowPlugin {
            primary_window: None,
            exit_condition: bevy::window::ExitCondition::DontExit,
            ..Default::default()
        })
    );

    editor_app.add_plugins(bevy_egui::EguiPlugin);
    
    editor_app.add_systems(Startup, test_systems::setup);
    editor_app.add_systems(Update, test_systems::camera_rotate);

    editor_app.add_systems(
            PreStartup,
            test_systems::configure_context.after(bevy_egui::EguiStartupSet::InitContexts),
        )
        .add_systems(Update, test_systems::ui_example_system);

    let mut winit_app = WinitApp {
        editor_app,
        ..Default::default()
    };
    event_loop.run_app(&mut winit_app).unwrap();
}
