///unit struct
use crate::circle::Circle;

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

///elastic collisons conserve momentum and energies, readjust velocities cosidering unit masses
pub fn collide(circ1: &Circle, circ2: &Circle) -> [f64; 4] {
    // just exchange of velocities happen (in both directions)
    return [circ2.vel_x, circ2.vel_y, circ1.vel_x, circ1.vel_y];
}
