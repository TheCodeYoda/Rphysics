use crate::circle::Circle;
use glm::*;
use nalgebra_glm as glm;

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let sq = ((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1));
    return sq.sqrt();
}

pub fn is_colliding(circ1: &Circle, circ2: &Circle) -> bool {
    let dist = distance(circ1.x(), circ1.y(), circ2.x(), circ2.y());
    if (dist - (circ1.r() + circ2.r())) < 0.7 {
        return true;
    }
    return false;
}

/// elastic collisons conserve momentum and energies, readjust velocities
pub fn collide(circ1: &Circle, circ2: &Circle) -> ((f64, f64), (f64, f64)) {
    //Conserve energy and momentum look at wikipedia for elastic collisons
    let total_mass = circ1.mass + circ2.mass;
    let mass_ratio_1 = (2.0 * circ2.mass) / total_mass;
    let mass_ratio_2 = (2.0 * circ1.mass) / total_mass;

    let v1 = circ1.v;
    let v2 = circ2.v;
    let x1 = circ1.point;
    let x2 = circ2.point;

    let dot_1 = dot(&(v1 - v2), &(x1 - x2));
    let v1_new = circ1.v - (mass_ratio_1 * (dot_1 / length2(&(x1 - x2))) * (x1 - x2));

    let dot_2 = dot(&(v2 - v1), &(x2 - x1));
    let v2_new = circ2.v - (mass_ratio_2 * (dot_2 / length2(&(x2 - x1))) * (x2 - x1));

    return ((v1_new[0], v1_new[1]), (v2_new[0], v2_new[1]));
}
