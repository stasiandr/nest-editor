pub mod view;
pub mod user_project;


use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowWrapper};
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};
use bevy_egui::{EguiContext, EguiContextSettings, EguiFullOutput, EguiInput};
use winit::keyboard::Key;
use winit::{event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};


#[derive(Default)]
struct WinitApp {
    editor_window_entity: Option<Entity>,
    game_window_entity: Option<Entity>,
    editor_app: App,
    game_app: Option<App>,
    target: bool,
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

        self.window_handle = Some(bevy::window::RawHandleWrapper::new(wrapper).unwrap());

        self.editor_app.insert_non_send_resource(windows);
        self.editor_app.finish();
        self.editor_app.cleanup();
        self.editor_window_entity = Some(entity);

        self.window_id = Some(editor_window_id);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        
        let mut win_entity = self.editor_app.world_mut().entity_mut(self.editor_window_entity.unwrap());
        let mut win = win_entity.get_mut::<bevy::window::Window>().unwrap();
        
        
        match event {
            WindowEvent::Resized(size) => {
                win.resolution.set_physical_resolution(size.width, size.height);

                // window_resized.write(WindowResized {
                //     window: window_entity,
                //     width: window.width(),
                //     height: window.height(),
                // });
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
                    window: self.editor_window_entity.unwrap(),
                    position,
                    delta,
                };
                self.editor_app.world_mut().send_event(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.editor_app.world_mut().send_event(bevy::input::mouse::MouseButtonInput {
                    button: convert_mouse_button(button),
                    state: convert_element_state(state),
                    window: self.editor_window_entity.unwrap(),
                });
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    return;
                }

                println!("{:?}", event.logical_key);

                if let Key::Named(winit::keyboard::NamedKey::Backspace) = event.logical_key {

                    std::process::Command::new("cargo")
                        .current_dir(user_project::load_dylib_path().parent().unwrap())
                        .arg("build").arg("--lib")
                        .output()
                        .expect("Failed to build project");
                        

                    self.destroy_game();
                    self.load_game();

                    println!("Game loaded");
                    return;
                }

                println!("Switching targets");

                self.target = !self.target;

                if !self.target {
                    let (res , _) = self.game_app.as_mut().unwrap().world_mut().query::<(Entity, &bevy::window::RawHandleWrapper)>().single(self.game_app.as_mut().unwrap().world_mut());
                    self.game_app.as_mut().unwrap().world_mut().entity_mut(res).remove::<bevy::window::RawHandleWrapper>();

                    let (res , _) = self.editor_app.world_mut().query::<(Entity, &bevy::window::PrimaryWindow)>().single(self.editor_app.world_mut());
                    self.editor_app.world_mut().entity_mut(res).insert(self.window_handle.as_ref().unwrap().clone());
                } else {
                    let (res , _) = self.editor_app.world_mut().query::<(Entity, &bevy::window::RawHandleWrapper)>().single(self.editor_app.world_mut());
                    self.editor_app.world_mut().entity_mut(res).remove::<bevy::window::RawHandleWrapper>();

                    let (res , _) = self.game_app.as_mut().unwrap().world_mut().query::<(Entity, &bevy::window::PrimaryWindow)>().single(self.game_app.as_mut().unwrap().world_mut());
                    self.game_app.as_mut().unwrap().world_mut().entity_mut(res).insert(self.window_handle.as_ref().unwrap().clone());
                }
            }
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if self.target {
                    if  let Some(game_app) = self.game_app.as_mut() {
                        game_app.update();    
                    }
                }
                
                self.editor_app.update();

                let window = self.editor_app.world().non_send_resource::<bevy_winit::WinitWindows>().get_window(self.editor_window_entity.unwrap());
                window.unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl WinitApp {
    fn destroy_game(&mut self) {
        self.game_app = None;
    }

    fn load_game(&mut self) {

        let mut game_app = user_project::load_app_builder_from_dylib(user_project::load_dylib_path());
        
        let mut windows = bevy_winit::WinitWindows::default();
        let entity = game_app.world_mut().spawn_empty().id();

        windows.entity_to_winit.insert(entity, self.window_id.unwrap());
        windows.winit_to_entity.insert(self.window_id.unwrap(), entity);

        let mut e = game_app.world_mut().entity_mut(entity);
        e.insert(PrimaryWindow);
        e.insert(bevy::window::Window::default());

        game_app.insert_non_send_resource(windows);
        game_app.finish();
        game_app.cleanup();

        game_app.update();
        
        self.game_window_entity = Some(entity);
        self.game_app = Some(game_app);
    }
}



// Test two app simultaneously
fn main() {

    let mut game_app = user_project::load_app_builder_from_dylib(user_project::load_dylib_path());
    game_app.finish();
    game_app.cleanup();
    game_app.update();

    return;
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
    // editor_app.add_plugins(view::NestEditorViewPlugin);
    
    editor_app.add_systems(Startup, setup);
    editor_app.add_systems(Update, camera_rotate);

    editor_app.add_systems(
            PreStartup,
            configure_context.after(bevy_egui::EguiStartupSet::InitContexts),
        )
        .add_systems(Update, ui_example_system);

    // let mut game_app = App::new();

    // game_app.add_plugins(DefaultPlugins.build()
    //     .disable::<bevy::winit::WinitPlugin>()
    //     .set(WindowPlugin {
    //         primary_window: None,
    //         exit_condition: bevy::window::ExitCondition::DontExit,
    //         ..Default::default()
    //     })
    // );

    // // game_app.add_plugins(bevy_egui::EguiPlugin);

    // game_app.add_systems(Startup, setup_game);
    // game_app.add_systems(Update, camera_rotate);

    let mut winit_app = WinitApp {
        editor_app,
        game_app: None,
        ..Default::default()
    };
    event_loop.run_app(&mut winit_app).unwrap();
}







pub fn convert_mouse_button(mouse_button: winit::event::MouseButton) -> MouseButton {
    match mouse_button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Back => MouseButton::Back,
        winit::event::MouseButton::Forward => MouseButton::Forward,
        winit::event::MouseButton::Other(val) => MouseButton::Other(val),
    }
}

pub fn convert_element_state(element_state: winit::event::ElementState) -> bevy::input::ButtonState {
    match element_state {
        winit::event::ElementState::Pressed => bevy::input::ButtonState::Pressed,
        winit::event::ElementState::Released => bevy::input::ButtonState::Released,
    }
}









fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &bevy::window::Window)>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let window = q.single().0;

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            target: bevy::render::camera::RenderTarget::Window(bevy::window::WindowRef::Entity(window)),
            ..Default::default()
        }
    ));
}


fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_rotate(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Camera3d)>,
) {
    for (mut t, _) in q.iter_mut() {
        t.rotate(Quat::from_rotation_y(ops::sin_cos(time.delta_secs()).0 * 0.1));
    }

}






fn configure_context(mut egui_settings: Query<&mut EguiContextSettings>) {
    for mut es in egui_settings.iter_mut() {
        es.run_manually = true;
        // println!("{:#?}", es);
    }
}

fn ui_example_system(
    mut contexts: Query<(&mut EguiContext, &mut EguiInput, &mut EguiFullOutput)>
) {
    let (mut ctx, mut egui_input, mut egui_full_output) = contexts.single_mut();

    let ui = |ctx: &bevy_egui::egui::Context| {
        bevy_egui::egui::Window::new("Hello").show(ctx, |ui| {
            
            if (ui.button("Run game")).clicked() {
                println!("Button clicked!");
            }
        });
    };

    let ctx = ctx.get_mut();

    **egui_full_output = Some(ctx.run(egui_input.take(), ui));
}