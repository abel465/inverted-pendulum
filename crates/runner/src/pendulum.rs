use glam::*;
use std::time::Duration;

pub struct Pendulum {
    pub cart_pos: Vec2,
    pub bob_pos: Vec2,
    cart_linvel: f32,
    bob_angvel: f32,
}

const CART_SPEED: f32 = 0.5;
const GRAVITY: f32 = -0.1;
const RADIUS: f32 = 0.4;

impl Pendulum {
    pub fn new() -> Self {
        Self {
            cart_pos: Vec2::ZERO,
            cart_linvel: 0.0,
            bob_pos: RADIUS * Vec2::Y,
            bob_angvel: 0.0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn move_left(&mut self) {
        self.cart_linvel = -CART_SPEED;
    }

    pub fn move_right(&mut self) {
        self.cart_linvel = CART_SPEED;
    }

    pub fn stop(&mut self) {
        self.cart_linvel = 0.0;
    }

    pub fn update(&mut self, delta: Duration) {
        let factor = delta.as_secs_f32();

        // Cart velocity contribution
        self.cart_pos.x = (self.cart_pos.x + self.cart_linvel * factor).clamp(-0.5, 0.5);

        // Cart pull on bob
        if (self.cart_pos.distance(self.bob_pos) - RADIUS).abs() > 0.0001 {
            let relative_pos = self.cart_pos - self.bob_pos;
            let new_relative_pos = relative_pos.normalize() * RADIUS;
            let dif = relative_pos - new_relative_pos;
            self.bob_angvel += dif.x.atan2(dif.y) * factor * 0.05;
            self.bob_pos = self.cart_pos - new_relative_pos;
        }
        let angle = self.bob_angle();

        // Gravity
        self.bob_angvel += GRAVITY * angle.sin() * factor;

        // Friction
        self.bob_angvel *= 1.0 - 0.15 * factor;

        // Angular velocity contribution
        self.bob_pos = self.cart_pos - RADIUS * Vec2::from((angle + self.bob_angvel).sin_cos());
    }

    fn bob_angle(&self) -> f32 {
        let pos = self.cart_pos - self.bob_pos;
        pos.x.atan2(pos.y)
    }
}
