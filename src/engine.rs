use crate::circle::Circle;
use crate::collison::*;
use crate::screen::Screen;

extern crate nalgebra_glm as glm;
use glm::*;

fn intersects(a: &DVec2, b: &DVec2) -> bool {
    if f64::min(b[0], b[1]) < f64::max(a[0], a[1]) {
        return true;
    }
    return false;
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
        self.g = vec2(0.0, 30.0);
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

    fn resolve_active_collisons(&mut self, active_list: &Vec<usize>, e: f64) {
        let n = active_list.len();
        for i in 0..n {
            for j in i + 1..n {
                let ind_i = active_list[i];
                let ind_j = active_list[j];
                if let Some(p) = self.object_list[ind_i].is_colliding(&self.object_list[ind_j]) {
                    // resolve static collison for ith and jth circle
                    self.object_list[ind_i].point = p.0;
                    self.object_list[ind_j].point = p.1;
                    // collide and readjust velocit for ith and jth sphere
                    let vel = self.object_list[ind_i].collide(&self.object_list[ind_j], e);
                    self.object_list[ind_i].v = vel.0;
                    self.object_list[ind_j].v = vel.1;
                }
            }
        }
    }

    // implementing sweep and prune algo
    pub fn resolve_collisons(&mut self) {
        let n = self.object_list.len();
        // let mut res = Vec::new();
        self.object_list
            .sort_by(|a, b| a.y().partial_cmp(&b.y()).unwrap());

        let mut active_list: Vec<usize> = Vec::new();
        active_list.push(0);

        for i in 1..n {
            if !active_list.is_empty() {
                let ind: usize = *active_list.last().unwrap();
                if intersects(&self.object_list[i].point, &self.object_list[ind].point) {
                    active_list.push(i);
                    self.resolve_active_collisons(&active_list, self.e);
                } else {
                    while !active_list.is_empty() {
                        active_list.pop();
                    }
                }
            } else {
                active_list.push(i);
            }
        }

        // for i in 0..n {
        //     for j in i + 1..n {
        //         if let Some(p) = self.object_list[i].is_colliding(&self.object_list[j]) {
        //             // resolve static collison for ith and jth circle
        //             self.object_list[i].point = p.0;
        //             self.object_list[j].point = p.1;
        //             // collide and readjust velocit for ith and jth sphere
        //             let vel = self.object_list[i].collide(&self.object_list[j], self.e());
        //             self.object_list[i].v = vel.0;
        //             self.object_list[j].v = vel.1;
        //         }
        //     }
        // }
    }
}
