use num::{Float, FromPrimitive, NumCast};
use std::f64::consts::PI;

/// A bearing is a direction between 0-2PI radians (0-360 degrees).
#[derive(Copy, Clone)]
pub struct Bearing<T: Float> {
    a: T,
}

impl<T: Float + FromPrimitive> Bearing<T> {
    fn tau() -> T {
        let two = FromPrimitive::from_u8(2).unwrap();
        Self::pi() * two
    }

    fn pi() -> T {
        FromPrimitive::from_f64(PI).unwrap()
    }

    pub fn new(mut angle: T) -> Bearing<T> {
        while angle < FromPrimitive::from_u8(0).unwrap() {
            angle = angle + Self::tau();
        }
        Bearing {
            a: angle % Self::tau(),
        }
    }

    pub fn radians(&self) -> T {
        self.a
    }

    /// Shortest angular distance between this bearing and another.
    pub fn distance(&self, other: &Bearing<T>) -> T {
        let n1: T = FromPrimitive::from_i8(-1).unwrap();
        let d = other.a - self.a;
        if d > Self::pi() {
            d - Self::tau()
        } else if d < n1 * Self::pi() {
            d + Self::tau()
        } else {
            d
        }
    }
}

impl<T: Float + FromPrimitive> std::ops::Add<T> for Bearing<T> {
    type Output = Bearing<T>;

    fn add(self, rhs: T) -> Self::Output {
        Bearing::<T>::new(self.a + rhs)
    }
}

impl<T: Float + FromPrimitive> std::ops::Sub<T> for Bearing<T> {
    type Output = Bearing<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Bearing::<T>::new(self.a - rhs)
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
            let a = Bearing::new(TAU + 1.234);
            assert!(approx_eq!(f32, a.radians(), 1.234));
        }

        #[test]
        fn test_constructor_clamp_negative() {
            let a = Bearing::new(-1.234);
            assert!(approx_eq!(f32, a.radians(), -1.234 + TAU));
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
            assert!(approx_eq!(f32, a2.radians(), 13.0 % TAU));
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
            assert!(approx_eq!(f32, a2.radians(), 13.0 % TAU));
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
            assert!(approx_eq!(f32, a1.distance(&a2), -TAU + 5.9))
        }

        #[test]
        fn test_positive_wrap() {
            let a1 = Bearing::new(6.0);
            let a2 = Bearing::new(0.1);
            assert!(approx_eq!(f32, a1.distance(&a2), (TAU - 6.0) + 0.1))
        }
    }
}
