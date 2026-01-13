use num::{Float, FromPrimitive};

pub struct Field<T> {
    width: T,
    height: T,
}

impl<T> Field<T>
where
    T: Float + FromPrimitive + Copy,
{
    pub fn new(width: T, height: T) -> Self {
        Field { width, height }
    }

    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }

    pub fn contains(&self, x: T, y: T) -> bool {
        let zero = T::zero();
        x >= zero && x <= self.width && y >= zero && y <= self.height
    }

    pub fn wrap(&self, x: T, y: T) -> (T, T) {
        let zero = T::zero();
        let x = if x < zero {
            self.width
        } else if x > self.width {
            zero
        } else {
            x
        };

        let y = if y < zero {
            self.height
        } else if y > self.height {
            zero
        } else {
            y
        };

        (x, y)
    }
}
