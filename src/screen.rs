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

// a trait for checking bounds of screen and readjusting the position of the shape accordingly
pub trait Check {
    fn check_bounds(&mut self, width: f64, height: f64);
}
