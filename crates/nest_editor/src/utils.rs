use bevy::input::mouse::MouseButton;


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


