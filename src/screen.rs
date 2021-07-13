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
            // other.point = vec2(width - other.r(), other.point[1]);
            return true;
        }
        // hits lower wall
        if other.point[1] + other.r() > height && other.v[1] > 0.0 {
            // other.point = vec2(other.point[0], height - other.r());
            return true;
        }
        // hits left side wall
        if other.point[0] - other.r() < 0.0 && other.v[0] < 0.0 {
            // other.point = vec2(other.r(), other.point[1]);
            return true;
        }
        // hits upper wall
        if other.point[1] - other.r() < 0.0 && other.v[1] < 0.0 {
            // other.point = vec2(other.point[1], other.r());
            return true;
        }
        return false;
    }

    fn collide(&mut self, other: &mut Circle, e: f64, dt: f64) {
        let rv = other.v;
        let mut poc = vec2(0.0, 0.0);
        if other.point[0] + other.r() > self.width {
            poc = vec2(other.point[0] + other.r(), other.point[1]);
        }
        // hits lower wall
        if other.point[1] + other.r() > self.height {
            poc = vec2(other.point[0], other.point[1] + other.r());
        }
        // hits left side wall
        if other.point[0] - other.r() < 0.0 {
            poc = vec2(other.point[0] - other.r(), other.point[1]);
        }
        // hits upper wall
        if other.point[1] - other.r() < 0.0 {
            poc = vec2(other.point[0], other.point[1] - other.r());
        }

        let normal = (other.point - poc) / length(&(other.point - poc));

        let vel_normal = dot(&rv, &normal);

        let j_mag = -vel_normal * (1.0 + e) / (1.0 / other.mass);
        let impulse = j_mag * normal;

        other.v = other.v + (1.0 / other.mass * impulse);
    }
}
