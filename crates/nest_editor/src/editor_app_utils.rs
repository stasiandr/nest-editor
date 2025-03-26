

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowWrapper};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;

impl crate::EditorApp {
    pub fn handle_mouse_move(&mut self, position: PhysicalPosition<f64>) {
        let mut win_entity = self.editor_app.world_mut().entity_mut(self.main_window.editor_window_entity.unwrap());
        let mut win = win_entity.get_mut::<bevy::window::Window>().unwrap();

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

    pub fn handle_window_resize(&mut self, size: PhysicalSize<u32>) {
        let mut win_entity = self.editor_app.world_mut().entity_mut(self.main_window.editor_window_entity.unwrap());
        let mut win = win_entity.get_mut::<bevy::window::Window>().unwrap();

        win.resolution.set_physical_resolution(size.width, size.height);
    }

    pub fn handle_scale_factor_changed(&mut self, scale_factor: f64) {
        let mut window_entity = self.editor_app.world_mut().entity_mut(self.main_window.editor_window_entity.unwrap());
        let mut window = window_entity.get_mut::<bevy::window::Window>().unwrap();

        window.resolution.set_scale_factor(scale_factor as f32);

        // self.editor_app.world_mut().send_event(WindowBackendScaleFactorChanged {
        //     window: window_entity,
        //     scale_factor,
        // });

        // let prior_factor = window.resolution.scale_factor();
        // let scale_factor_override = window.resolution.scale_factor_override();

        // if scale_factor_override.is_none() && !relative_eq!(scale_factor as f32, prior_factor) {
        //     self.editor_app.world_mut().send_event(WindowScaleFactorChanged {
        //         window: window_entity,
        //         scale_factor,
        //     });
        // }
    }

    pub fn handle_mouse_wheel(&mut self, delta: winit::event::MouseScrollDelta) {

        match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                let event = bevy::input::mouse::MouseWheel {
                    unit: bevy::input::mouse::MouseScrollUnit::Line,
                    x,
                    y,
                    window: self.main_window.editor_window_entity.unwrap(),
                };
                self.editor_app.world_mut().send_event(event);
            }
            winit::event::MouseScrollDelta::PixelDelta(p) => {
                let event = bevy::input::mouse::MouseWheel {
                    unit: bevy::input::mouse::MouseScrollUnit::Pixel,
                    x: p.x as f32,
                    y: p.y as f32,
                    window: self.main_window.editor_window_entity.unwrap(),
                };
                self.editor_app.world_mut().send_event(event);
            }
        }
    }

    pub fn insert_window_into_editor_app(&mut self, window: Window) {
        let main_window_id = window.id();

        let wrapper = self.windows
            .entry(main_window_id)
            .insert(WindowWrapper::new(window))
            .into_mut();

        let entity = self.editor_app.world_mut().spawn_empty().id();
        let mut e = self.editor_app.world_mut().entity_mut(entity);
        e.insert(PrimaryWindow);
        e.insert(bevy::window::Window::default());
        e.insert(bevy::window::RawHandleWrapper::new(wrapper).unwrap());

        self.main_window.window_handle = Some(bevy::window::RawHandleWrapper::new(wrapper).unwrap());

        self.editor_app.finish();
        self.editor_app.cleanup();
        self.main_window.editor_window_entity = Some(entity);

        self.main_window.window_id = Some(main_window_id);
    }

    pub fn remove_raw_handle_wrapper(&mut self) {
        let (res , _) = self.editor_app.world_mut().query::<(bevy::ecs::entity::Entity, &PrimaryWindow)>().single(self.editor_app.world_mut());
        self.editor_app.world_mut().entity_mut(res).remove::<bevy::window::RawHandleWrapper>();
    }

    pub fn insert_raw_handle_wrapper(&mut self) {
        let (res , _) = self.editor_app.world_mut().query::<(bevy::ecs::entity::Entity, &PrimaryWindow)>().single(self.editor_app.world_mut());
        self.editor_app.world_mut().entity_mut(res).insert(self.main_window.window_handle.as_ref().unwrap().clone());
    }
}