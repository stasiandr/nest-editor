pub mod view;
pub mod user_project;

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowWrapper};
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};
use winit::{event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};

// The main entry point for the application
// fn main() {
//     let mut editor_app = App::new();
//     editor_app
//         .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
//         .add_plugins(EguiPlugin)
//         .insert_resource(NestEditorState::default())
//         .insert_resource(view::TabViewer::default())
//         .add_systems(Update, view::editor_ui_update);
//     editor_app.run();
// }

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


        // Initializing child app

        let game_app = self.game_app.as_mut().unwrap();

        let mut windows = bevy_winit::WinitWindows::default();
        let entity = game_app.world_mut().spawn_empty().id();

        windows.entity_to_winit.insert(entity, editor_window_id);
        windows.winit_to_entity.insert(editor_window_id, entity);

        let mut e = game_app.world_mut().entity_mut(entity);
        e.insert(PrimaryWindow);
        e.insert(bevy::window::Window::default());
        // e.insert(handle_clone);

        game_app.insert_non_send_resource(windows);
        game_app.finish();
        game_app.cleanup();
        self.game_window_entity = Some(entity);


        self.window_id = Some(editor_window_id);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        
        match event {
            WindowEvent::KeyboardInput { event, .. } => {

                if event.state.is_pressed() {
                    return;
                }

                println!("Key pressed: {:?}", event);

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
                self.game_app.as_mut().unwrap().update();
                self.editor_app.update();

                // if !self.target{
                    let window = self.editor_app.world().non_send_resource::<bevy_winit::WinitWindows>().get_window(self.editor_window_entity.unwrap());
                    window.unwrap().request_redraw();
                // } else {
                //     let window = self.game_app.as_mut().unwrap().world().non_send_resource::<bevy_winit::WinitWindows>().get_window(self.game_window_entity.unwrap());
                //     window.unwrap().request_redraw();
                // }
            }
            _ => (),
        }
    }
}



// Test two app simultaneously
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
    
    editor_app.add_systems(Startup, setup);
    editor_app.add_systems(Update, camera_rotate);

    let mut game_app = App::new();

    game_app.add_plugins(DefaultPlugins.build()
        .disable::<bevy::winit::WinitPlugin>()
        .set(WindowPlugin {
            primary_window: None,
            exit_condition: bevy::window::ExitCondition::DontExit,
            ..Default::default()
        })
    );

    game_app.add_systems(Startup, setup_game);
    game_app.add_systems(Update, camera_rotate);

    let mut winit_app = WinitApp {
        editor_app,
        game_app: Some(game_app),
        ..Default::default()
    };
    event_loop.run_app(&mut winit_app).unwrap();
}


















fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &bevy::window::Window)>,
) {
    println!("startup worked");
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
    println!("startup worked");
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