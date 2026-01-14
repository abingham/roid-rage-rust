use glam::Vec2;
use rand::Rng;
use std::f32::consts::PI;

pub fn random_bearing() -> f32 {
    let mut rng = rand::rng();
    (rng.random::<f32>() * 2.0 - 1.0) * PI
}

/// Create a vector from quantity (e.g. speed) and bearing.
pub fn from_quantity_and_bearing(quantity: f32, bearing: f32) -> Vec2 {
    Vec2::new(bearing.cos(), bearing.sin()) * quantity
}

#[cfg(test)]
mod tests {
    use super::from_quantity_and_bearing;
    use float_cmp::approx_eq;
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn from_quantity_and_bearing_east() {
        let v = from_quantity_and_bearing(3.0, 0.0);
        assert!(approx_eq!(f32, v.x, 3.0, epsilon = 0.0001));
        assert!(approx_eq!(f32, v.y, 0.0, epsilon = 0.0001));
    }

    #[test]
    fn from_quantity_and_bearing_north() {
        let v = from_quantity_and_bearing(2.0, FRAC_PI_2);
        assert!(approx_eq!(f32, v.x, 0.0, epsilon = 0.0001));
        assert!(approx_eq!(f32, v.y, 2.0, epsilon = 0.0001));
    }
}
