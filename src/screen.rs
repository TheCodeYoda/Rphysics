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
