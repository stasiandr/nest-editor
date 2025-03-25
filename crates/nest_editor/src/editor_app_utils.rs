

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