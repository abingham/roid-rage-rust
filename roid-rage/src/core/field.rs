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

#[cfg(test)]
mod tests {
    use super::Field;

    #[test]
    fn contains_includes_bounds() {
        let field = Field::new(10.0_f32, 5.0_f32);
        assert!(field.contains(0.0, 0.0));
        assert!(field.contains(10.0, 5.0));
        assert!(!field.contains(-0.1, 1.0));
        assert!(!field.contains(1.0, 5.1));
    }

    #[test]
    fn wrap_resets_outside_edges() {
        let field = Field::new(10.0_f32, 5.0_f32);
        assert_eq!(field.wrap(-1.0, 2.0), (10.0, 2.0));
        assert_eq!(field.wrap(11.0, 2.0), (0.0, 2.0));
        assert_eq!(field.wrap(3.0, -1.0), (3.0, 5.0));
        assert_eq!(field.wrap(3.0, 6.0), (3.0, 0.0));
    }
}
