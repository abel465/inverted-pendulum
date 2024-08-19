use super::Agent;
use crate::pendulum::Pendulum;
use std::time::Duration;

#[derive(Clone)]
pub struct Inputs {
    pub cart_x: f32,
    pub bob_x: f32,
    pub bob_y: f32,
    pub angvel: f32,
}

impl super::Inputs for Inputs {
    const COUNT: usize = 4;

    fn get(&self, index: usize) -> f32 {
        match index {
            0 => self.cart_x,
            1 => self.bob_x,
            2 => self.bob_y,
            3 => self.angvel,
            _ => panic!(),
        }
    }
}

pub type PendulumAgent = Agent<Inputs, Outputs>;

#[derive(Clone)]
pub struct Outputs {
    pub speed: f32,
}

impl super::Outputs for Outputs {
    const COUNT: usize = 1;

    fn from_iter<I: Iterator<Item = f32>>(mut it: I) -> Self {
        Self {
            speed: it.next().unwrap(),
        }
    }
}

pub fn set_pendulum_inputs(pendulum: &mut Pendulum, agent: &mut PendulumAgent) {
    let outputs = agent.choose(Inputs {
        cart_x: pendulum.cart_x(),
        bob_x: pendulum.bob_pos().x,
        bob_y: pendulum.bob_pos().y,
        angvel: pendulum.angvel(),
    });
    let speed = outputs.speed;
    if speed > 0.1 {
        pendulum.move_right();
    } else if speed < -0.1 {
        pendulum.move_left();
    } else {
        pendulum.stop();
    }
}

pub fn run_simulation(agent: &mut PendulumAgent) -> f32 {
    let mut pendulum = Pendulum::new();
    let delta = Duration::from_secs_f64(1.0 / 30.0);
    let mut score = 0.0;
    for _ in 0..30 * 100 {
        set_pendulum_inputs(&mut pendulum, agent);
        pendulum.update(delta);
        let y = pendulum.bob_pos_normalized().y;
        if y > 0.9 {
            score += y / (pendulum.angvel().abs() * 4.0 + 1.0) / (1.0 + pendulum.cart_x().abs());
        }
    }
    score
}
