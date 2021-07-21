use crate::collison::{distance, Collision};

extern crate nalgebra_glm as glm;
use glm::*;
use rand::*;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Circle {
    pub point: DVec2,
    pub theta: f64,
    r: f64,
    pub v: DVec2,
    pub w: f64,
    pub force: DVec2,
    pub mass: f64,
    pub moment_of_inertia: f64,
    color: [f32; 4],
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        let mut rng = rand::thread_rng();
        let color = [
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            1.0,
        ];

        return Circle {
            point: vec2(x, y),
            theta: 0.0,
            r: r,
            v: vec2(0.0, 0.0),
            w: 0.0,
            force: vec2(0.0, 0.0),
            mass: PI * r * r,
            moment_of_inertia: PI * r * r * r * r / 2.0,
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

    pub fn theta(&self) -> f64 {
        return self.theta;
    }

    pub fn color(&self) -> [f32; 4] {
        return self.color;
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

    /// adds force
    pub fn add_force(&mut self, force: DVec2) {
        self.force += force;
    }
}

impl Collision for Circle {
    fn is_colliding(&mut self, other: &mut Circle) -> bool {
        let dist = distance(self.x(), self.y(), other.x(), other.y());
        if dist <= self.r() + other.r() {
            let overlap = 0.5 * (dist - self.r() - other.r());

            // resolve static collison by displacing them away iif they are overlapping
            // move self,other away by 0.5 of overlap in unit vector direction
            self.point = self.point - overlap * ((self.point - other.point) / dist);
            other.point = other.point + overlap * ((self.point - other.point) / dist);

            return true;
        }
        return false;
    }

    fn collide(&mut self, other: &mut Circle, e: f64, dt: f64) {
        // calculate relative velocity
        let rv = other.v - self.v;
        // caculate collison normal
        let normal = (other.point - self.point) / length(&(other.point - self.point));
        // calculate rv along normal
        let vel_normal = dot(&rv, &normal);
        // calculate impulse scalar
        let mut j = -(1.0 + e) * vel_normal;
        j = j / (1.0 / self.mass + 1.0 / other.mass);
        // apply impulse
        let impulse = j * normal;
        // F.dt = J
        // self.add_force(-impulse / dt);
        // other.add_force(impulse / dt);
        self.v = self.v - (1.0 / self.mass * impulse);
        other.v = other.v + (1.0 / other.mass * impulse);

        // angular momentum & impulse
        // calculate relative velocity
        let rv_angular = other.w - self.w;

        let mut j_angular = -(1.0 + e) * rv_angular;
        j_angular = j_angular / (1.0 / self.moment_of_inertia + 1.0 / other.moment_of_inertia);
        // apply impulse
        let angular_impulse = j_angular;
        // F.dt = J
        // self.add_force(-impulse / dt);
        // other.add_force(impulse / dt);
        self.w = self.w - (1.0 / self.moment_of_inertia * angular_impulse);
        other.w = other.w + (1.0 / other.moment_of_inertia * angular_impulse);
    }

    fn apply_impulse(&mut self, impulse: DVec2) {
        self.v = self.v - (1.0 / self.mass * impulse);
    }
}
