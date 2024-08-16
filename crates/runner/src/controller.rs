use crate::pendulum::Pendulum;
use shared::ShaderConstants;
use std::time::{Duration, Instant};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton},
    keyboard::{Key, NamedKey},
};

pub struct Controller {
    prev_instant: Instant,
    mouse_button_pressed: u32,
    cursor_x: f32,
    cursor_y: f32,
    pendulum: Pendulum,
    current_direction: Option<NamedKey>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            prev_instant: Instant::now(),
            mouse_button_pressed: 0,
            cursor_x: 0.0,
            cursor_y: 0.0,
            pendulum: Pendulum::new(),
            current_direction: None,
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

    pub fn on_key_press(&mut self, logical_key: Key, state: ElementState) {
        match logical_key {
            Key::Named(NamedKey::ArrowLeft) => {
                if state.is_pressed() {
                    self.current_direction = Some(NamedKey::ArrowLeft);
                    self.pendulum.move_left();
                } else if self
                    .current_direction
                    .is_some_and(|d| d == NamedKey::ArrowLeft)
                {
                    self.pendulum.stop();
                }
            }
            Key::Named(NamedKey::ArrowRight) => {
                if state.is_pressed() {
                    self.current_direction = Some(NamedKey::ArrowRight);
                    self.pendulum.move_right();
                } else if self
                    .current_direction
                    .is_some_and(|d| d == NamedKey::ArrowRight)
                {
                    self.pendulum.stop();
                }
            }
            Key::Character(str) if str == "r" => self.pendulum.reset(),
            _ => {}
        }
    }

    pub fn update(&mut self) {
        let max_duration = Duration::from_secs_f64(1.0 / 30.0);
        let now = Instant::now();
        let duration = (now - self.prev_instant).min(max_duration);
        self.pendulum.update(duration);
        self.prev_instant = now;
    }

    pub fn shader_constants(&self, window_size: PhysicalSize<u32>) -> ShaderConstants {
        ShaderConstants {
            width: window_size.width,
            height: window_size.height,
            cursor_x: self.cursor_x,
            cursor_y: self.cursor_y,
            mouse_button_pressed: self.mouse_button_pressed,
            cart_x: self.pendulum.cart_x(),
            bob_x: self.pendulum.bob_pos().x,
            bob_y: self.pendulum.bob_pos().y,
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
