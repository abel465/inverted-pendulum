use crate::pendulum::Pendulum;
use shared::ShaderConstants;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton},
    keyboard::Key,
};

pub struct Controller {
    mouse_button_pressed: u32,
    cursor_x: f32,
    cursor_y: f32,
    pendulum: Pendulum,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            mouse_button_pressed: 0,
            cursor_x: 0.0,
            cursor_y: 0.0,
            pendulum: Pendulum::new(),
        }
    }

    pub fn on_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        let mask = 1 << mouse_button_index(button);
        match state {
            ElementState::Pressed => self.mouse_button_pressed |= mask,
            ElementState::Released => self.mouse_button_pressed &= !mask,
        }
    }

    pub fn on_mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_x = position.x as f32;
        self.cursor_y = position.y as f32;
    }

    pub fn on_key_press(&mut self, _logical_key: Key, _state: ElementState) {}

    pub fn update(&mut self) {
        self.pendulum.update();
    }

    pub fn shader_constants(&self, window_size: PhysicalSize<u32>) -> ShaderConstants {
        ShaderConstants {
            width: window_size.width,
            height: window_size.height,
            cursor_x: self.cursor_x,
            cursor_y: self.cursor_y,
            mouse_button_pressed: self.mouse_button_pressed,
            position: self.pendulum.position as f32,
            angle: self.pendulum.angle as f32,
        }
    }
}

fn mouse_button_index(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
        MouseButton::Back => 3,
        MouseButton::Forward => 4,
        MouseButton::Other(i) => 5 + (i as usize),
    }
}
