pub struct Pendulum {
    pub position: f64,
    pub speed: f64,
    pub angle: f64,
}

impl Pendulum {
    pub fn new() -> Self {
        Self {
            position: 0.0,
            speed: 0.0,
            angle: 0.0,
        }
    }

    pub fn update(&mut self) {}
}
