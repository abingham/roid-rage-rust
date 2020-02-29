#[derive(Clone, Copy)]
pub struct Field {
    width: usize,
    height: usize,
}

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
        x >= 0.0
            && x <= self.width() as f32
            && y >= 0.0
            && y <= self.height() as f32
    }

    // TODO: This should accept and number type
    pub fn wrap(&self, x: f32, y: f32) -> (f32, f32) {
        let x = if x < 0.0 {
            self.width() as f32
        }
        else if x > self.width() as f32 {
            0.0
        }
        else {
            x
        };

        let y = if y < 0.0 {
            self.height() as f32
        }
        else if y > self.height() as f32 {
            0.0
        }
        else {
            y
        };

        (x, y)
    }
}
