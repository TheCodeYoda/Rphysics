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
            // circ.point[0] += circ.v[0] * dt;
            // // v = u+at;
            // if circ.point[1] + circ.r() < self.screen.height() {
            //     circ.v[1] = circ.v[1] + self.g * dt;
            // }
            // circ.point[1] += circ.v[1] * dt;
            // println!("{:?}", (circ.point));
            let normal_reaction = circ.check_bounds(&self.screen, self.e);
            let net_force = circ.force + normal_reaction;
            // println!("{:?}", (circ.force, net_force));
            let acc = net_force / circ.mass;
            circ.v = circ.v + (acc * dt);
            circ.point = circ.point + (circ.v * dt);
        }
    }

    pub fn check_border(&mut self) {
        for circ in &mut self.object_list {
            circ.check_bounds(&self.screen, self.e);
        }
    }
    // implementing sweep and prune algo
    pub fn resolve_collisons(&mut self, dt: f64) {
        let n = self.object_list.len();
        // sort according to x axis
        // self.object_list
        //     .sort_by(|a, b| (a.x() - a.r()).partial_cmp(&(b.x() - b.r())).unwrap());
        // active list contains circles which are overlapping in specified axis
        for i in 0..n {
            for j in i + 1..n {
                let (left, right) = self.object_list.split_at_mut(j);
                resolve_active_collisons(&mut left[i], &mut right[0], self.e, dt);
            }
        }
    }
}
