use crate::circle::Circle;

extern crate nalgebra_glm as glm;

/// finds euclidian distance between 2 points
pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let sq = ((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1));
    return sq.sqrt();
}

// a trait for checking bounds of screen and readjusting the position of the shape accordingly
pub trait Collision {
    fn is_colliding(&mut self, other: &mut Circle) -> bool;
    fn collide(&mut self, other: &mut Circle, e: f64, dt: f64);
}
