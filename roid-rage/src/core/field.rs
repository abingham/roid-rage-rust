pub struct Field {
    width: usize,
    height: usize,
}

// TODO: Does Field need to be a trait, e.g. for use with both
// our Field struct as well as for the grpc one?
impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        Field {
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= 0.0 && x <= self.width() as f32 && y >= 0.0 && y <= self.height() as f32
    }

    // TODO: This should accept any number type
    pub fn wrap(&self, x: f32, y: f32) -> (f32, f32) {
        let x = if x < 0.0 {
            self.width() as f32
        } else if x > self.width() as f32 {
            0.0
        } else {
            x
        };

        let y = if y < 0.0 {
            self.height() as f32
        } else if y > self.height() as f32 {
            0.0
        } else {
            y
        };

        (x, y)
    }
}
