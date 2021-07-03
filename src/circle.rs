use crate::screen::Check;

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

    pub fn disp_coords(&self) {
        println!("x: {} y: {}", self.x, self.y);
    }
}

impl Check for Circle {
    fn check_bounds(&mut self, width: f64, height: f64) {
        if self.x + self.r > width {
            self.x = width - self.r;
            self.vel_x = 0.0;
        }
        if self.y + self.r > height {
            self.y = height - self.r;
            self.vel_y = 0.0;
        }
        if self.x - self.r < 0.0 {
            self.x = self.r;
            self.vel_x = 0.0;
        }
        if self.y - self.r < 0.0 {
            self.y = self.r;
            self.vel_y = 0.0;
        }
    }
}
