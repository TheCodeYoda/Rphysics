use crate::circle::Circle;
use glm::*;

extern crate nalgebra_glm as glm;

/// finds euclidian distance between 2 points
pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let sq = ((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1));
    sq.sqrt()
}

// a trait for collison
pub trait Collision {
    fn is_colliding(&mut self, other: &mut Circle) -> bool;
    fn collide(&mut self, other: &mut Circle, e: f64, dt: f64);
    fn apply_impulse(&mut self, impulse: DVec2, poa: DVec2);
}

// a trait for friction
pub trait Friction {
    fn add_friction_impulse(&mut self, dynamic_friction: f64, g: DVec2);
    fn add_rolling_friction_impulse(&mut self, other: &mut Circle, g: DVec2);
}
