use super::{user_lib_wrapper::UserLibWrapper, UserProject};


#[derive(Default)]
pub struct UserApp {
    app: Option<*mut bevy::app::App>,
    lib: Option<UserLibWrapper>,
    state: UserAppState,
}

impl UserApp {
    pub fn load_lib(&mut self, user_project: &UserProject) {
        if !self.state.is(UserAppState::Uninitialized){
            panic!("Library already loaded");
        }

        self.lib = Some(UserLibWrapper::new(user_project));
        self.state = UserAppState::LibraryLoaded;
    }

    pub fn unload_lib(&mut self) {
        if !self.state.is(UserAppState::LibraryLoaded) {
            panic!("Library not loaded");
        }

        self.lib.take().unwrap().unload_dylib();
        self.state = UserAppState::Uninitialized;
    }

    pub fn get_app_state(&self) -> &UserAppState {
        &self.state
    }

    pub fn build_app(&mut self) {
        if !self.state.at_least(UserAppState::LibraryLoaded) {
            panic!("Library not loaded");
        }

        self.app = Some(self.lib.as_ref().unwrap().app_builder());
        self.state = UserAppState::GameAppBuilt;
    }

    pub fn update_app(&self) {
        if !self.state.at_least(UserAppState::GameAppBuilt) {
            panic!("Game app not built");
        }

        self.lib.as_ref().unwrap().update_app(self.app.unwrap());
    }

    pub fn pass_window(&mut self, raw_handle_wrapper: bevy::window::RawHandleWrapper) {
        if !self.state.at_least(UserAppState::GameAppBuilt) {
            panic!("Game app not built");
        }

        let app_kit_handle = nest_editor_shared::raw_pointer_from_handle_wrapper(raw_handle_wrapper);
        self.lib.as_ref().unwrap().set_window_handle_from_app_kit(self.app.unwrap(), app_kit_handle);

        self.state = UserAppState::WindowPassedToGame;
    }

    pub fn remove_window(&mut self) {
        if !self.state.at_least(UserAppState::WindowPassedToGame) {
            panic!("Window not passed to game")
        }

        self.lib.as_ref().unwrap().remove_window_handle(self.app.unwrap());
        self.state = UserAppState::GameAppBuilt;
    }

    pub fn kill_app(&mut self) {
        if !self.state.at_least(UserAppState::GameAppBuilt) {
            panic!("Game app not built");
        }

        self.app = None;
        self.state = UserAppState::LibraryLoaded;
    }

    pub fn is_back_to_editor_requested(&self) -> bool {
        if !self.state.at_least(UserAppState::GameAppBuilt) {
            panic!("Game app not built");
        }

        self.lib.as_ref().unwrap().is_back_to_editor_requested(self.app.unwrap())
    }

    pub fn handle_window_resize(&self, size: winit::dpi::PhysicalSize<u32>) {
        if !self.state.at_least(UserAppState::WindowPassedToGame) {
            panic!("Game app not built");
        }

        self.lib.as_ref().unwrap().handle_window_resize(self.app.unwrap(), size.width, size.height);
    }

    pub fn handle_mouse_input(&self, mouse_input: &bevy::input::mouse::MouseButtonInput) {
        if !self.state.at_least(UserAppState::WindowPassedToGame) {
            panic!("Game app not built");
        }

        let json_serialized = serde_json::to_string(mouse_input).unwrap();
        let cstr = std::ffi::CString::new(json_serialized).unwrap();
        self.lib.as_ref().unwrap().handle_mouse_input(self.app.unwrap(), cstr.as_ptr());
    }
    
    pub(crate) fn handle_mouse_move(&self, position: winit::dpi::PhysicalPosition<f64>)  {
        if !self.state.at_least(UserAppState::WindowPassedToGame) {
            panic!("Game app not built");
        }

        self.lib.as_ref().unwrap().handle_mouse_move(self.app.unwrap(), position.x, position.y);
    }

    
}


#[derive(Debug, Default, PartialEq, Eq)]
pub enum UserAppState {
    #[default]
    Uninitialized,
    LibraryLoaded,
    GameAppBuilt,
    WindowPassedToGame,
}

impl UserAppState {
    #[inline]
    pub fn at_least(&self, input: Self) -> bool {
        match input {
            UserAppState::Uninitialized => true,
            UserAppState::LibraryLoaded => matches!(self, UserAppState::LibraryLoaded | UserAppState::GameAppBuilt | UserAppState::WindowPassedToGame),
            UserAppState::GameAppBuilt => matches!(self, UserAppState::GameAppBuilt | UserAppState::WindowPassedToGame),
            UserAppState::WindowPassedToGame => matches!(self, UserAppState::WindowPassedToGame),
        }
    }

    #[inline]
    pub fn is(&self, input: Self) -> bool {
        self == &input
    }
}