use crate::circle::Circle;
use crate::engine_traits::*;
use crate::screen::Screen;

use rand::*;
extern crate nalgebra_glm as glm;
use glm::*;

fn resolve_active_collisons(circ_1: &mut Circle, circ_2: &mut Circle, e: f64, dt: f64) {
    if circ_1.is_colliding(circ_2) {
        // collide and readjust velocit for ith and jth sphere
        circ_1.collide(circ_2, e, dt);
    }
}

pub struct Engine {
    g: DVec2,
    e: f64,
    pub object_list: Vec<Circle>,
    pub screen: Screen,
}

impl Engine {
    pub fn new(e: f64, screen: Screen) -> Engine {
        return Engine {
            g: vec2(0.0, 0.0),
            e: e,
            object_list: Vec::new(),
            screen: screen,
        };
    }

    pub fn g(&self) -> DVec2 {
        return self.g;
    }

    pub fn e(&self) -> f64 {
        return self.e;
    }

    pub fn gravity_on(&mut self) {
        self.g = vec2(0.0, 100.0);
        for circ in self.object_list.iter_mut() {
            circ.force += circ.mass * self.g;
        }
    }

    pub fn gravity_off(&mut self) {
        self.g = vec2(0.0, 0.0)
    }

    pub fn add(&mut self, circ: Circle) {
        self.object_list.push(circ);
    }

    /// randomly spawns Non overlapping circles in the Screen
    pub fn get_circle(&mut self) -> Option<Circle> {
        let mut tries = 100000;
        while tries > 0 {
            // rand handle
            let mut rng = rand::thread_rng();

            let _min_vel = 600.0;
            let _max_vel = 700.0;
            let min_radius = 25.0;
            let max_radius = 30.0;
            let width = self.screen.width();
            let height = self.screen.height();

            let mut circ = Circle::new(
                rng.gen_range(max_radius, width - max_radius),
                rng.gen_range(max_radius, height - max_radius),
                rng.gen_range(min_radius, max_radius),
            );
            let mut flag = 1;
            for sample in self.object_list.iter_mut() {
                if circ.is_colliding(sample) {
                    flag = 0;
                    break;
                }
            }

            if flag == 1 {
                return Some(circ);
            }

            tries -= 1;
        }
        None
    }

    pub fn get_circles(&mut self, n: u32) {
        // let w = 512.0;
        // let h = 512.0;
        // let v = 100.0;
        // let circ_1 = Circle::new(256.0, 0.0, 50.0, 0.0, v);
        // let x = w/2.0 -((w/2.0)*(PI/3.0).sin());
        // let y = ((w/2.0*(PI/3.0).cos())) + h/2.0;
        // let circ_2 = Circle::new(x, y, 50.0,v*(PI/6.0).cos(), -v*(PI/6.0).sin());
        // let x = w/2.0 +((w/2.0)*(PI/3.0).sin());
        // let circ_3 = Circle::new(x, y, 50.0, -v*(PI/6.0).cos(), -v*(PI/6.0).sin());
        // return vec![circ_1, circ_2,circ_3];

        // ------------------custom testing ----------------------------
        // let circ_1 = Circle::new(256.0, 256.0, 50.0, 60.0, 0.0, 40.0);
        // let _circ_2 = Circle::new(462.0, 50.0, 50.0, -100.0, 0.0, 0.0);
        // engine.object_list = vec![circ_1];

        // --------------------------random testing -------------------------------------
        for _i in 0..n {
            if let Some(circ) = self.get_circle() {
                self.object_list.push(circ);
            }
        }
    }

    pub fn mouse_impulse(&mut self, start_point: DVec2, curr_pos: DVec2) {
        let mut chosen_circ: Circle = Circle::new(0.0, 0.0, 0.0);
        let mut flag = 0;
        for circ in &mut self.object_list {
            if circ.point[0] - circ.r() <= start_point[0]
                && circ.point[0] + circ.r() >= start_point[0]
                && circ.point[1] - circ.r() <= start_point[1]
                && circ.point[1] + circ.r() >= start_point[1]
            {
                flag = 1;
                chosen_circ = *circ;
                break;
            }
        }
        if flag == 1 {
            let impulse = curr_pos - start_point;
            self.object_list.retain(|x| x.point != chosen_circ.point);
            chosen_circ.v = vec2(0.0, 0.0);
            chosen_circ.apply_impulse(impulse * 5000.0, start_point);
            self.add(chosen_circ);
        }
    }

    pub fn update_pos(&mut self, dt: f64) {
        for circ in &mut self.object_list {
            // linear motion
            circ.add_force(circ.mass * self.g);
            let net_force = circ.force;
            let acc = net_force / circ.mass;
            circ.v += acc * dt;
            circ.point += circ.v * dt;
            // println!("vel: {} acc: {}", length2(&circ.v), length2(&acc));

            //angular motion
            let net_torque = circ.torque;
            let angular_acc = net_torque / circ.moment_of_inertia;
            circ.w += angular_acc * dt;
            circ.theta = (circ.theta + (circ.w * dt)) % 360.0;
            // println!("{} {}", net_torque, circ.w);

            // clamping velocities
            if (circ.v[0] * circ.v[0]) + (circ.v[1] * circ.v[1]) <= 0.42 {
                circ.v = vec2(0.0, 0.0);
            }
            if circ.w * circ.w <= 0.42 {
                circ.w = 0.0;
            }

            //resetting force & torque
            circ.force = vec2(0.0, 0.0);
            circ.torque = 0.0;
        }
    }

    pub fn resolve_wall_collisions(&mut self, dt: f64) {
        for circ in &mut self.object_list {
            if self.screen.is_colliding(circ) {
                self.screen.collide(circ, self.e, dt);
            }
        }
    }
    // implementing sweep and prune algo
    pub fn resolve_collisons(&mut self, dt: f64) {
        let n = self.object_list.len();
        // sort according to x axis
        // self.object_list
        //     .sort_by(|a, b| (a.x() - a.r()).partial_cmp(&(b.x() - b.r())).unwrap());
        // active list contains circles which are overlapping in specified axis
        (0..n).for_each(|i| {
            (i + 1..n).for_each(|j| {
                let (left, right) = self.object_list.split_at_mut(j);
                resolve_active_collisons(&mut left[i], &mut right[0], self.e, dt);
            });
        });
    }
}
