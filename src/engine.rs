use crate::circle::Circle;
use crate::collison::*;
use crate::screen::Screen;

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

    pub fn add_gravity_force(&mut self) {
        for circ in self.object_list.iter_mut() {
            circ.force += circ.mass * self.g;
        }
    }

    pub fn remove_gravity_force(&mut self) {
        for circ in self.object_list.iter_mut() {
            circ.force -= circ.mass * self.g;
        }
    }

    pub fn gravity_off(&mut self) {
        self.g = vec2(0.0, 0.0)
    }

    pub fn add(&mut self, circ: Circle) {
        self.object_list.push(circ);
    }

    pub fn update_pos(&mut self, dt: f64) {
        for circ in &mut self.object_list {
            // println!("{:?}", (circ.force, dt));
            let net_force = circ.force + circ.mass * self.g;
            let acc = net_force / circ.mass;
            circ.v = circ.v + (acc * dt);
            circ.point = circ.point + (circ.v * dt);
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
