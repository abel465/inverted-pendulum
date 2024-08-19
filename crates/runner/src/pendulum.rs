use glam::*;
use std::time::Duration;

pub struct Pendulum {
    cart_x: f32,
    cart_linvel: f32,
    cart_linacc: f32,
    bob_angvel: f32,
    bob_angle: f32,
}

const CART_MAX_SPEED: f32 = 1.0;
const CART_ACC: f32 = 4.0;
const CART_FRICTION: f32 = 2.0;
const GRAVITY: f32 = -9.81;
const RADIUS: f32 = 0.4;
const MAX_X: f32 = 0.5;
const MIN_X: f32 = -0.5;

impl Pendulum {
    pub fn new() -> Self {
        Self {
            cart_x: 0.0,
            cart_linvel: 0.0,
            cart_linacc: 0.0,
            bob_angvel: 0.0,
            bob_angle: 0.0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn move_left(&mut self) {
        self.cart_linacc = -CART_ACC;
    }

    pub fn move_right(&mut self) {
        self.cart_linacc = CART_ACC;
    }

    pub fn stop(&mut self) {
        self.cart_linacc = 0.0;
    }

    pub fn update(&mut self, delta: Duration) {
        let delta_secs = delta.as_secs_f32();

        // Cart velocity
        let old_v = self.cart_linvel;
        self.cart_linvel = (old_v + self.cart_linacc * delta_secs)
            .clamp(
                (MIN_X - self.cart_x) / delta_secs,
                (MAX_X - self.cart_x) / delta_secs,
            )
            .clamp(-CART_MAX_SPEED, CART_MAX_SPEED);
        let dvel = self.cart_linvel - old_v;
        self.cart_x += self.cart_linvel * delta_secs;

        // Angular velocity
        self.bob_angvel +=
            (dvel * self.bob_angle.cos() + GRAVITY * self.bob_angle.sin() * delta_secs) / RADIUS
                * 0.005; // WHY DO I NEED THIS???
        self.bob_angle += self.bob_angvel;

        // Friction
        self.bob_angvel *= 1.0 - 0.01 * delta_secs;
        self.cart_linvel = (self.cart_linvel.abs() - CART_FRICTION * delta_secs)
            .max(0.0)
            .copysign(self.cart_linvel);
    }

    pub fn cart_x(&self) -> f32 {
        self.cart_x
    }

    pub fn bob_pos(&self) -> Vec2 {
        Vec2::X * self.cart_x - Vec2::from((self.bob_angle).sin_cos()) * RADIUS
    }

    pub fn bob_pos_normalized(&self) -> Vec2 {
        Vec2::X * self.cart_x - Vec2::from((self.bob_angle).sin_cos())
    }

    pub fn angvel(&self) -> f32 {
        self.bob_angvel
    }
}
