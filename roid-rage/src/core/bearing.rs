use std::f32::consts::PI;

type Angle = f32;

/// A bearing is a direction between 0-2PI radians (0-360 degrees).
#[derive(Copy, Clone)]
pub struct Bearing {
    a: Angle,
}

impl Bearing {
    pub fn new(mut angle: Angle) -> Bearing {
        while angle < 0.0 {
            angle += 2.0 * PI;
        }
        Bearing {
            a: angle % (2.0 * PI),
        }
    }

    pub fn radians(&self) -> f32 {
        self.a
    }

    /// Shortest angular distance between this bearing and another.
    pub fn distance(&self, other: &Bearing) -> Angle {
        let d = other.a - self.a;
        if d > PI {
            d - 2.0 * PI
        }
        else if d < -1.0 * PI {
            d + 2.0 * PI
        }
        else {
            d
        }
        
    }
}

impl std::ops::Add<Angle> for Bearing {
    type Output = Bearing;

    fn add(self, rhs: Angle) -> Self::Output {
        Bearing::new(self.a + rhs)
    }
}

impl std::ops::Sub<Angle> for Bearing {
    type Output = Bearing;

    fn sub(self, rhs: f32) -> Self::Output {
        Bearing::new(self.a - rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    mod construct {
        use super::*;

        #[test]
        fn test_constructor_clamp_positive() {
            let a = Bearing::new(2.0 * PI + 1.234);
            assert!(approx_eq!(f32, a.radians(), 1.234));
        }

        #[test]
        fn test_constructor_clamp_negative() {
            let a = Bearing::new(-1.234);
            assert!(approx_eq!(f32, a.radians(), -1.234 + 2.0 * PI));
        }
    }

    mod add {
        use super::*;
        #[test]
        fn test_add() {
            let a1 = Bearing::new(1.0);
            let a2 = a1 + 1.23;
            assert_eq!(a2.radians(), 2.23);
        }

        #[test]
        fn test_add_positive_clamp() {
            let a1 = Bearing::new(1.0);
            let a2 = a1 + 12.0;
            assert!(approx_eq!(f32, a2.radians(), 13.0 % (2.0 * PI)));
        }

        #[test]
        fn test_add_negative_clamp() {
            let a1 = Bearing::new(1.0);
            let a2 = a1 + -12.0;
            assert!(approx_eq!(f32, a2.radians(), -11.0 + 4.0 * PI));
        }
    }

    mod subtract {
        use super::*;
        #[test]
        fn test_subtract() {
            let a1 = Bearing::new(3.0);
            let a2 = a1 - 2.0;
            assert_eq!(a2.radians(), 1.0);
        }

        #[test]
        fn test_subtract_positive_clamp() {
            let a1 = Bearing::new(1.0);
            let a2 = a1 - -12.0;
            assert!(approx_eq!(f32, a2.radians(), 13.0 % (2.0 * PI)));
        }

        #[test]
        fn test_subtract_negative_clamp() {
            let a1 = Bearing::new(1.0);
            let a2 = a1 - 12.0;
            assert!(approx_eq!(f32, a2.radians(), -11.0 + 4.0 * PI));
        }
    }

    mod distance {
        use super::*;

        #[test]
        fn test_positive() {
            let a1 = Bearing::new(0.5);
            let a2 = Bearing::new(1.5);
            assert!(approx_eq!(f32, a1.distance(&a2), 1.0));
        }

        #[test]
        fn test_negative() {
            let a1 = Bearing::new(1.0);
            let a2 = Bearing::new(0.5);
            assert!(approx_eq!(f32, a1.distance(&a2), -0.5));
        }

        #[test]
        fn test_negative_wrap() {
            let a1 = Bearing::new(0.1);
            let a2 = Bearing::new(6.0);
            assert!(approx_eq!(f32, a1.distance(&a2), -2.0 * PI + 5.9))
        }

        #[test]
        fn test_positive_wrap() {
            let a1 = Bearing::new(6.0);
            let a2 = Bearing::new(0.1);
            assert!(approx_eq!(f32, a1.distance(&a2), (2.0 * PI - 6.0) + 0.1))
        }
    }
}
