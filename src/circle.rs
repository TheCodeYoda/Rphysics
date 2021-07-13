use crate::collison::{distance, Collision};
use crate::screen::Screen;

extern crate nalgebra_glm as glm;
use glm::*;
use rand::*;
use std::f64::consts::PI;

pub struct Circle {
    pub point: DVec2,
    r: f64,
    pub v: DVec2,
    pub force: DVec2,
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
            force: vec2(0.0, 0.0),
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
    // pub fn update_pos(&mut self, dt: f64, grav: &Gravity, screen: &Screen) {
    //     self.point[0] += self.v[0] * dt;
    //     // v = u+at;
    //     if self.point[1] + self.r < screen.height() {
    //         self.v[1] = self.v[1] + grav.g() * dt;
    //     }
    //     self.point[1] += self.v[1] * dt;
    // }

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
    /// consider collison with the wall
    fn check_bounds(&mut self, screen: &Screen, e: f64) -> DVec2 {
        let width = screen.width();
        let height = screen.height();
        // hits right side wall
        if self.point[0] + self.r > width && self.v[0] >= 0.0 {
            self.v[0] = -e * self.v[0];
            self.point[0] = width - self.r;
            return vec2(-self.force[0], self.force[1]);
        }
        // hits lower wall
        if self.point[1] + self.r > height && self.v[1] >= 0.0 {
            self.v[1] = -e * self.v[1];
            self.point[1] = height - self.r;
            return vec2(self.force[0], -self.force[1]);
            // let net_force = self.mass * self.a;
            // let normal_reaction = -net_force;
            // self.a = (net_force + normal_reaction) / self.mass;
        }
        // hits left side wall
        if self.point[0] - self.r < 0.0 && self.v[0] <= 0.0 {
            self.v[0] = -e * self.v[0];
            self.point[0] = self.r;
            return vec2(-self.force[0], self.force[1]);
        }
        // hits upper wall
        if self.point[1] - self.r < 0.0 && self.v[1] <= 0.0 {
            self.v[1] = -e * self.v[1];
            self.point[1] = self.r;
            return vec2(self.force[0], -self.force[1]);
        }
        return vec2(0.0, 0.0);
    }

    fn is_colliding(&self, other: &Circle) -> Option<(DVec2, DVec2)> {
        let dist = distance(self.x(), self.y(), other.x(), other.y());
        if dist <= self.r() + other.r() {
            let overlap = 0.5 * (dist - self.r() - other.r());

            // resolve static collison by displacing them away iif they are overlapping
            // move self,other away by 0.5 of overlap in unit vector direction
            let self_point = self.point - overlap * ((self.point - other.point) / dist);
            let other_point = other.point + overlap * ((self.point - other.point) / dist);

            return Some((self_point, other_point));
        }
        return None;
    }

    fn collide(&self, other: &Circle, e: f64) -> (DVec2, DVec2) {
        //Conserve energy and momentum look at wikipedia for elastic collisons
        // let total_mass = self.mass + other.mass;
        // let mass_ratio_1 = (2.0 * other.mass) / total_mass;
        // let mass_ratio_2 = (2.0 * self.mass) / total_mass;

        // let v1 = self.v;
        // let v2 = other.v;
        // let x1 = self.point;
        // let x2 = other.point;

        // let dot_1 = dot(&(v1 - v2), &(x1 - x2));
        // let self_v = e * (self.v - (mass_ratio_1 * (dot_1 / length2(&(x1 - x2))) * (x1 - x2)));

        // let dot_2 = dot(&(v2 - v1), &(x2 - x1));
        // let other_v = e * (other.v - (mass_ratio_2 * (dot_2 / length2(&(x2 - x1))) * (x2 - x1)));

        // return (self_v, other_v);

        // calculate relative velocity
        let rv = other.v - self.v;

        let normal = (other.point - self.point) / length(&(other.point - self.point));

        // calculate rv along normal
        let vel_normal = dot(&rv, &normal);

        // do not resolve if velocities are separating
        // if vel_normal > 0.0 {
        //     return (self.v, other.v);
        // }

        // calculate impulse scalar
        let mut j = -(1.0 + e) * vel_normal;
        j /= 1.0 / self.mass + 1.0 / other.mass;

        //apply impulse
        let impulse = j * normal;
        let vel_self = self.v - ((1.0 / self.mass) * impulse);
        let vel_other = other.v + ((1.0 / other.mass) * impulse);
        println!("{:?}", (length(&normal), vel_self, vel_other));
        return (vel_self, vel_other);
    }
}
