use crate::gravity::*;
use crate::screen::*;

extern crate nalgebra_glm as glm;
use glm::*;
use rand::*;
use std::f64::consts::PI;

pub struct Circle {
    pub point: DVec2,
    r: f64,
    pub v: DVec2,
    pub mass: f64,
    color: [f32; 4],
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64, vel_x: f64, vel_y: f64) -> Circle {
        let mut rng = rand::thread_rng();
        let color = [
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            1.0,
        ];

        return Circle {
            point: vec2(x, y),
            r: r,
            v: vec2(vel_x, vel_y),
            mass: PI * r * r,
            color: color,
        };
    }
    pub fn x(&self) -> f64 {
        return self.point[0];
    }

    pub fn y(&self) -> f64 {
        return self.point[1];
    }

    pub fn r(&self) -> f64 {
        return self.r;
    }

    pub fn vel_x(&self) -> f64 {
        return self.v[0];
    }

    pub fn vel_y(&self) -> f64 {
        return self.v[1];
    }

    pub fn color(&self) -> [f32; 4] {
        return self.color;
    }

    /// update coords given time
    pub fn update_pos(&mut self, dt: f64, grav: &Gravity, screen: &Screen) {
        self.point[0] += self.v[0] * dt;
        // v = u+at;
        if self.point[1] + self.r < screen.height() {
            self.v[1] = self.v[1] + grav.g() * dt;
        }
        self.point[1] += self.v[1] * dt;
    }

    /// helper to display coords
    pub fn disp_coords(&self) {
        println!("x: {} y: {}", self.point[0], self.point[1]);
    }

    /// readjusts x,y,r to render out in piston's ellipse func
    pub fn readjust(&self) -> [f64; 4] {
        return [
            self.point[0] - self.r,
            self.point[1] - self.r,
            (self.r) * 2.0,
            (self.r) * 2.0,
        ];
    }
}

impl Check for Circle {
    /// consider collison with the wall
    fn check_bounds(&mut self, width: f64, height: f64) {
        let e = 0.83;
        // hits right side wall
        if self.point[0] + self.r > width && self.v[0] > 0.0 {
            self.v[0] = -e * self.v[0];
        }
        // hits lower wall
        if self.point[1] + self.r > height && self.v[1] > 0.0 {
            self.v[1] = -e * self.v[1];
        }
        // hits left side wall
        if self.point[0] - self.r < 0.0 && self.v[0] < 0.0 {
            self.v[0] = -e * self.v[0];
        }
        // hits upper wall
        if self.point[1] - self.r < 0.0 && self.v[1] < 0.0 {
            self.v[1] = -e * self.v[1];
        }
    }
}
