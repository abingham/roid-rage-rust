pub fn to_vector(radians: f32) -> glam::Vec2 {
    let x = radians.cos();
    let y = radians.sin();
    glam::Vec2::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::super::to_vector;
    use float_cmp::ApproxEqRatio;
    use std::f32::consts::PI;

    #[test]
    fn east() {
        let east = 0.0;
        let expected = glam::Vec2::new(1.0, 0.0);
        assert_eq!(to_vector(east), expected);
    }

    #[test]
    fn west() {
        let west = PI;
        let actual = to_vector(west);
        let expected = glam::Vec2::new(-1.0, 0.0);
        assert!(actual.x.approx_eq_ratio(&expected.x, 0.0001));
        assert!((actual.y - expected.y).abs() < 0.00001);

        // TODO: Why doesn't this work? And the others?
        // assert!(actual.y.approx_eq_ratio(&expected.y, 0.01));
    }

    #[test]
    fn north() {
        let north = PI / 2.0;
        let actual = to_vector(north);
        let expected = glam::Vec2::new(0.0, 1.0);
        // assert_eq!(actual, expected);
        assert!((actual.x - expected.x).abs() < 0.0001);
        assert!((actual.y - expected.y).abs() < 0.0001);
    }

    #[test]
    fn south() {
        let south = 3.0 * PI / 2.0;
        let actual = to_vector(south);
        let expected = glam::Vec2::new(0.0, -1.0);
        assert!((actual.x - expected.x).abs() < 0.0001);
        assert!((actual.y - expected.y).abs() < 0.0001);
    }
}
