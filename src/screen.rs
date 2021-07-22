use crate::circle::Circle;
use crate::collison::Collision;

extern crate nalgebra_glm as glm;
use glm::*;

pub struct Screen {
    width: f64,
    height: f64,
}

impl Screen {
    pub fn new(width: f64, height: f64) -> Screen {
        return Screen {
            width: width,
            height: height,
        };
    }

    pub fn height(&self) -> f64 {
        return self.height;
    }

    pub fn width(&self) -> f64 {
        return self.width;
    }
}

impl Collision for Screen {
    fn is_colliding(&mut self, other: &mut Circle) -> bool {
        let width = self.width();
        let height = self.height();
        // hits right side wall
        if other.point[0] + other.r() > width && other.v[0] > 0.0 {
            return true;
        }
        // hits lower wall
        if other.point[1] + other.r() > height && other.v[1] > 0.0 {
            return true;
        }
        // hits left side wall
        if other.point[0] - other.r() < 0.0 && other.v[0] < 0.0 {
            return true;
        }
        // hits upper wall
        if other.point[1] - other.r() < 0.0 && other.v[1] < 0.0 {
            return true;
        }
        return false;
    }

    fn collide(&mut self, other: &mut Circle, e: f64, dt: f64) {
        fn apply_impulse(other: &mut Circle, poc: DVec2, e: f64) {
            let rv = other.v;
            let normal = (other.point - poc) / length(&(other.point - poc));

            let vel_normal = dot(&rv, &normal);

            let j_mag = -vel_normal * (1.0 + e) / (1.0 / other.mass);
            let impulse = j_mag * normal;
            // println!("{:?}", (other.v, impulse, vel_normal));
            other.v = other.v + (1.0 / other.mass * impulse);
            // angular momentum & impulse
            // calculate relative velocity
            let rv_angular = other.w;

            let mut j_angular = -(1.0 + e) * rv_angular;
            j_angular = j_angular / (1.0 / other.moment_of_inertia);
            // apply impulse
            let angular_impulse = j_angular;
            other.w = other.w + (1.0 / other.moment_of_inertia * angular_impulse);
        }

        if other.point[0] + other.r() > self.width {
            let poc = vec2(other.point[0] + other.r(), other.point[1]);
            other.point = vec2(self.width - other.r(), other.point[1]);
            apply_impulse(other, poc, e);
        }
        // hits lower wall
        if other.point[1] + other.r() > self.height {
            let poc = vec2(other.point[0], other.point[1] + other.r());
            other.point = vec2(other.point[0], self.height - other.r());
            apply_impulse(other, poc, e);
        }
        // hits left side wall
        if other.point[0] - other.r() < 0.0 {
            let poc = vec2(0.0, other.point[1]);
            other.point = vec2(other.r(), other.point[1]);
            apply_impulse(other, poc, e);
        }
        // hits upper wall
        if other.point[1] - other.r() < 0.0 {
            let poc = vec2(other.point[0], 0.0);
            other.point = vec2(other.point[0], other.r());
            apply_impulse(other, poc, e);
        }
    }

    fn apply_impulse(&mut self, _impulse: DVec2, _poa: DVec2) {}
}
