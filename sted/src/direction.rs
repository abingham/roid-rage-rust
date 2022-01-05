use num::{Float, FromPrimitive};

/// Vector representing a direction.
pub trait Direction<T: Float + FromPrimitive> {
    fn dx(&self) -> T;
    fn dy(&self) -> T;

    fn create(x: T, y: T) -> Self;

    /// Get the bearing of the direction in radians.
    /// 
    /// CCW azimuthal direction.
    fn bearing(&self) -> T {
        let x = self.dx();
        // NB: We flip the y axis because our coord. system is
        // different from the math one. I.e. for us -y is "up".
        let y = T::from(-1.0_f32).unwrap() * self.dy();
        T::atan2(y, x)
    }

    fn rotate(&self, amount: T) -> Self where Self: Sized {
        let new_brg = self.bearing() + amount; 
        let dx = new_brg.tan();
        Self::create(dx, T::from(-1.0_f32).unwrap()) 
    }
}

impl Direction<f32> for glam::Vec2 {
    fn create(x: f32, y: f32) -> Self {
        glam::Vec2::new(x, y)
    }

    fn dx(&self) -> f32 {
        self.x
    }

    fn dy(&self) -> f32 {
        self.y
    }
}

#[cfg(test)]
mod tests {

    mod add {
        use super::super::Direction;
        use std::f32::consts::PI;

        #[test]
        fn east_to_north() {
            let east = glam::Vec2::new(1.0, 0.0);
            let north = east.rotate(PI);
            let expected = glam::Vec2::new(0.0, -1.0);
            assert!((north.x - expected.x).abs() < 0.0000001);
        }
    }

    mod sub {
        use super::super::Direction;
        use std::f32::consts::PI;

        #[test]
        fn north_to_east() {
            let north = glam::Vec2::new(0.0, -1.0);
            let actual = north.rotate(-1.0 * PI).normalize();
            let expected = glam::Vec2::new(1.0, 0.0);
            assert!((actual.x - expected.x).abs() < 0.00000001);
        }
    }

    mod bearing {
        use super::super::Direction;
        use float_cmp::ApproxEqRatio;
        use std::f32::consts::PI;

        #[test]
        fn east() {
            let direction = glam::Vec2::new(1.0, 0.0);
            assert_eq!(direction.bearing(), 0.0);
        }

        #[test]
        fn west() {
            let direction = glam::Vec2::new(-1.0, 0.0);
            let brg = direction.bearing();
            assert!(brg.approx_eq_ratio(&(-1.0 * PI), 0.0001));
        }

        #[test]
        fn north() {
            let direction = glam::Vec2::new(0.0, -1.0);
            assert_eq!(direction.bearing(), PI / 2.0);
        }

        #[test]
        fn south() {
            let direction = glam::Vec2::new(0.0, 1.0);
            let brg = direction.bearing();
            assert_eq!(brg, -1.0 * PI / 2.0);
        }
    }
}
