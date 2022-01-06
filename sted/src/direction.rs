// TODO: Ok, this is cute but probably overkill. Why don't
// we just have a free "to_vector" function? Think this over.

pub trait Direction {
    fn radians(&self) -> &f32;

    fn vector(&self) -> glam::Vec2 {
        let x = self.radians().cos();
        let y = self.radians().sin();
        glam::Vec2::new(x, -1.0 * y)
    }

    // Get the bearing of the direction in radians.
    //
    // CCW azimuthal direction.
    // fn bearing(&self) -> f32 {
    //     let x = self.dx();
    //     // NB: We flip the y axis because our coord. system is
    //     // different from the math one. I.e. for us -y is "up".
    //     let y = -1.0 * self.dy();
    //     f32::atan2(y, x)
    // }

    // fn rotate(&self, amount: f32) -> Self where Self: Sized {
    //     // TODO: This is nuts. Should we just hardcode things to f32?

    //     let theta = self.bearing() + amount;

    //     let len = (self.dx().powf(2.0) + self.dy().powf(2.0)).sqrt();
    //     let new_x = theta.cos() * len;
    //     let new_y = theta.sin() * len;
    //     let new = Self::create(new_x, 1.0 * new_y);
    //     println!("{:?} {:?}", new_x, new_y);
    //     new
    // }
}

impl Direction for f32 {
    fn radians(&self) -> &f32 {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::super::Direction;
    use float_cmp::ApproxEqRatio;
    use std::f32::consts::PI;

    #[test]
    fn east() {
        let east = 0.0;
        let expected = glam::Vec2::new(1.0, 0.0);
        assert_eq!(east.vector(), expected);
    }

    #[test]
    fn west() {
        let west = PI;
        let actual = west.vector();
        let expected = glam::Vec2::new(-1.0, 0.0);
        assert!(actual.x.approx_eq_ratio(&expected.x, 0.0001));
        assert!((actual.y - expected.y).abs() < 0.00001);

        // TODO: Why doesn't this work? And the others?
        // assert!(actual.y.approx_eq_ratio(&expected.y, 0.01));
    }

    #[test]
    fn north() {
        let north = PI / 2.0;
        let actual = north.vector();
        let expected = glam::Vec2::new(0.0, -1.0);
        assert!((actual.x - expected.x).abs() < 0.0001);
        assert!((actual.y - expected.y).abs() < 0.0001);
    }

    #[test]
    fn south() {
        let south = 3.0 * PI / 2.0;
        let actual = south.vector();
        let expected = glam::Vec2::new(0.0, 1.0);
        assert!((actual.x - expected.x).abs() < 0.0001);
        assert!((actual.y - expected.y).abs() < 0.0001);
    }
}
