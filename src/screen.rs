use crate::circle::Circle;
use crate::collison::Collision;

extern crate nalgebra_glm as glm;
use glm::*;

pub struct Screen {
    width: f64,
    height: f64,
    poc: DVec2,
}

impl Screen {
    pub fn new(width: f64, height: f64) -> Screen {
        return Screen {
            width: width,
            height: height,
            poc: vec2(0.0, 0.0),
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
        if other.point[0] + other.r() > width {
            self.poc = vec2(other.point[0] + other.r(), other.point[1]);
            return true;
        }
        // hits lower wall
        if other.point[1] + other.r() > height {
            self.poc = vec2(other.point[0], other.point[1] + other.r());
            return true;
        }
        // hits left side wall
        if other.point[0] - other.r() < 0.0 {
            self.poc = vec2(other.point[0] - other.r(), other.point[1]);
            return true;
        }
        // hits upper wall
        if other.point[1] - other.r() < 0.0 {
            self.poc = vec2(other.point[0], other.point[1] - other.r());
            return true;
        }
        return false;
    }

    fn collide(&mut self, other: &mut Circle, e: f64, dt: f64) {
        let rv = other.v;

        let normal = (self.poc - other.point) / length(&(self.poc - other.point));

        let vel_normal = dot(&rv, &normal);

        let j_mag = -vel_normal * (1.0 + e) / (1.0 / other.mass);
        let impulse = j_mag * normal;

        other.v = other.v + (1.0 / other.mass * impulse);
    }
}
