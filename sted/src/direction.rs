use num::{Float, FromPrimitive};

/// Vector representing a direction.
pub trait Direction {
    fn dx(&self) -> f32;
    fn dy(&self) -> f32;

    fn create(x: f32, y: f32) -> Self;

    /// Get the bearing of the direction in radians.
    /// 
    /// CCW azimuthal direction.
    fn bearing(&self) -> f32 {
        let x = self.dx();
        // NB: We flip the y axis because our coord. system is
        // different from the math one. I.e. for us -y is "up".
        let y = -1.0 * self.dy();
        f32::atan2(y, x)
    }

    fn rotate(&self, amount: f32) -> Self where Self: Sized {
        // TODO: This is nuts. Should we just hardcode things to f32?

        let theta = self.bearing() + amount;

        let len = (self.dx().powf(2.0) + self.dy().powf(2.0)).sqrt();
        let new_x = theta.cos() * len;
        let new_y = theta.sin() * len;
        let new = Self::create(new_x, 1.0 * new_y);
        println!("{:?} {:?}", new_x, new_y);
        new
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
            let north = east.rotate(PI / 2.0);
            let expected = glam::Vec2::new(0.0, -1.0);
            assert_eq!(north.bearing(), expected.bearing());
        }

        #[test]
        fn north_to_east() {
            let north = glam::Vec2::new(0.0, -1.0);
            let east = north.rotate(-1.0 * PI / 2.0);
            let expected = glam::Vec2::new(1.0, 0.0);
            assert_eq!(east.bearing(), expected.bearing());
        }

        #[test]
        fn add_zero() {
            let original = glam::Vec2::new(-1.509958e-7, 1.0);
            let actual = original.rotate(0.0);
            assert_eq!(actual.bearing(), original.bearing());
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
