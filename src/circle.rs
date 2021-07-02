pub struct Circle {
    x: f64,
    y: f64,
    r: f64,
    vel_x: f64,
    vel_y: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64, vel_x: f64, vel_y: f64) -> Circle {
        return Circle {
            x: x,
            y: y,
            r: r,
            vel_x: vel_x,
            vel_y: vel_y,
        };
    }
    pub fn x(&self) -> f64 {
        return self.x;
    }

    pub fn y(&self) -> f64 {
        return self.y;
    }

    pub fn r(&self) -> f64 {
        return self.r;
    }

    pub fn vel_x(&self) -> f64 {
        return self.vel_x;
    }
    pub fn vel_y(&self) -> f64 {
        return self.vel_y;
    }

    pub fn update_pos(&mut self, dt: f64) {
        self.x += self.vel_x * dt;
        self.y += self.vel_y * dt;
    }
}
