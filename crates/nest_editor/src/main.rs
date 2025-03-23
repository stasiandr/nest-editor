pub mod view;
pub mod user_project;
pub mod utils;
pub mod test_systems;
pub mod winit_app_utils;


use bevy::prelude::*;
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};
use winit::{event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};


#[derive(Default)]
pub struct WinitApp {
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
        self.insert_window_into_editor_app(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        
        let mut win_entity = self.editor_app.world_mut().entity_mut(self.main_window.editor_window_entity.unwrap());
        let mut win = win_entity.get_mut::<bevy::window::Window>().unwrap();
        
        match event {
            WindowEvent::Resized(size) => {
                win.resolution.set_physical_resolution(size.width, size.height);
            }

            WindowEvent::CursorMoved { device_id: _, position } => {
                self.handle_mouse_move(position);
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
    nest_editor_shared::test();

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