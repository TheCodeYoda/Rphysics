use crate::circle::Circle;
use crate::collison::*;
use crate::screen::Screen;

pub struct Engine {
    g: f64,
    e: f64,
    pub object_list: Vec<Circle>,
    pub screen: Screen,
}

impl Engine {
    pub fn new(e: f64, screen: Screen) -> Engine {
        return Engine {
            g: 30.0,
            e: e,
            object_list: Vec::new(),
            screen: screen,
        };
    }

    pub fn g(&self) -> f64 {
        return self.g;
    }

    pub fn e(&self) -> f64 {
        return self.e;
    }

    pub fn gravity_on(&mut self) {
        self.g = 30.0;
    }

    pub fn gravity_off(&mut self) {
        self.g = 0.0;
    }

    pub fn add(&mut self, circ: Circle) {
        self.object_list.push(circ);
    }

    pub fn update_pos(&mut self, dt: f64) {
        for circ in &mut self.object_list {
            circ.point[0] += circ.v[0] * dt;
            // v = u+at;
            if circ.point[1] + circ.r() < self.screen.height() {
                circ.v[1] = circ.v[1] + self.g * dt;
            }
            circ.point[1] += circ.v[1] * dt;
        }
    }

    pub fn check_border(&mut self) {
        for circ in &mut self.object_list {
            circ.check_bounds(&self.screen, self.e);
        }
    }

    pub fn resolve_collisons(&mut self) {
        let n = self.object_list.len();
        // let mut res = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                if let Some(p) = self.object_list[i].is_colliding(&self.object_list[j]) {
                    // resolve static collison for ith and jth circle
                    self.object_list[i].point = p.0;
                    self.object_list[j].point = p.1;
                    // collide and readjust velocit for ith and jth sphere
                    let vel = self.object_list[i].collide(&self.object_list[j], self.e());
                    self.object_list[i].v = vel.0;
                    self.object_list[j].v = vel.1;
                }
            }
        }
    }
}
