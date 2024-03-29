use crate::engine_traits::{distance, Collision, Friction};

extern crate nalgebra_glm as glm;
use glm::*;
use rand::*;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Circle {
    pub id: u32,
    pub point: DVec2,
    pub theta: f64,
    r: f64,
    pub v: DVec2,
    pub w: f64,
    pub force: DVec2,
    pub torque: f64,
    pub mass: f64,
    pub moment_of_inertia: f64,
    rolling_friction_coeff: f64,
    color: [f32; 4],
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64, id: u32) -> Circle {
        let mut rng = rand::thread_rng();
        let color = [
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            1.0,
        ];

        return Circle {
            id: id,
            point: vec2(x, y),
            theta: 0.0,
            r: r,
            v: vec2(0.0, 0.0),
            w: 0.0,
            force: vec2(0.0, 0.0),
            torque: 0.0,
            mass: PI * r * r,
            moment_of_inertia: PI * r * r * r * r / 2.0,
            rolling_friction_coeff: 0.2,
            color: color,
        };
    }
    pub fn x(&self) -> f64 {
        self.point[0]
    }

    pub fn y(&self) -> f64 {
        self.point[1]
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn vel_x(&self) -> f64 {
        self.v[0]
    }

    pub fn vel_y(&self) -> f64 {
        self.v[1]
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
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

    /// adds torque
    pub fn add_torque(&mut self, torque: f64) {
        self.torque += torque;
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
        false
    }

    fn collide(&mut self, other: &mut Circle, e: f64, _dt: f64) {
        // calculate relative velocity
        let rv = other.v - self.v;
        // caculate collison normal
        let normal = (other.point - self.point) / length(&(other.point - self.point));
        // calculate rv along normal
        let vel_normal = dot(&rv, &normal);
        // calculate impulse scalar
        let mut j = -(1.0 + e) * vel_normal;
        j /= 1.0 / self.mass + 1.0 / other.mass;
        // apply impulse
        let impulse = j * normal;
        self.v -= 1.0 / self.mass * impulse;
        other.v += 1.0 / other.mass * impulse;

        // adjusting angular velocities according to the formula v = rw
        self.w -= length(&(1.0 / self.mass * impulse)) / self.r;
        other.w += length(&(1.0 / other.mass * impulse)) / other.r;
    }

    fn apply_impulse(&mut self, impulse: DVec2, poa: DVec2) {
        self.v -= 1.0 / self.mass * impulse;
        let r_vec = poa - self.point;
        let dir = cross2d(&r_vec, &self.v);
        let w_scalar = length(&self.v) / length(&r_vec);
        if dir < 0.0 {
            self.w += 10.0 * -w_scalar;
        } else {
            self.w += 10.0 * w_scalar;
        }
    }
}

impl Friction for Circle {
    fn add_friction_impulse(&mut self, dynamic_friction: f64, g: DVec2) {
        // if body is moving
        if length2(&self.v) > 0.0 {
            let normal_reaction = self.mass * length(&g);
            let vel_x = |x_vel: f64| -> f64 {
                if x_vel == 0.0 {
                    return 0.0;
                } else if x_vel < 0.0 {
                    return 1.0;
                }
                -1.0
            };
            // friction_force
            let dir_vec = vec2(vel_x(self.v[0]), 0.0);
            let force_friction = dynamic_friction * normal_reaction * dir_vec;
            self.add_force(force_friction);
            // friction_torque
            let r_vec = vec2(self.point[0], self.point[1] + self.r) - self.point;
            let torque_friction = cross2d(&r_vec, &force_friction);
            self.add_torque(torque_friction * 10000.0);
        }
    }

    fn add_rolling_friction_impulse(&mut self, _other: &mut Circle, g: DVec2) {
        // let dir = |w|-> f64 {
        //     if w>0.0 {
        //         1.0
        //     }
        //     else if w<0.0 {
        //         -1.0
        //     }
        //     0.0;
        // };

        // let dir_vec = vec2(dir(self.w))
    }
}
