pub struct Gravity {
    g: f64,
}

impl Gravity {
    pub fn new() -> Gravity {
        return Gravity { g: 0.0 };
    }

    pub fn g(&self) -> f64 {
        return self.g;
    }

    pub fn on(&mut self) {
        self.g = 30.0;
    }

    pub fn off(&mut self) {
        self.g = 0.0;
    }
}
