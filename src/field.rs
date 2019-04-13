use nalgebra::Point2;

pub struct Field {
    width: usize,
    height: usize,
    fringe: usize, 
}

impl Field {
    pub fn new(width: usize, height: usize, fringe: usize) -> Field {
        Field {
            width: width,
            height: height,
            fringe: fringe,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fringe(&self) -> usize {
        self.fringe
    }

    fn left_limit(&self) -> isize {
        -1 * (self.fringe as isize)
    }

    fn right_limit(&self) -> usize {
        self.width + self.fringe
    }

    fn bottom_limit(&self) -> isize {
        -1 * (self.fringe as isize)
    }

    fn top_limit(&self) -> usize {
        self.height + self.fringe
    }

    pub fn contains(&self, point: &Point2<f64>) -> bool {
        point.x >= self.left_limit() as f64
            && point.x <= self.right_limit() as f64
            && point.y >= self.bottom_limit() as f64
            && point.y <= self.top_limit() as f64
    }

    pub fn wrap(&self, point: &Point2<f64>) -> Point2<f64> {
        let x = if point.x < self.left_limit() as f64 {
            self.right_limit() as f64
        }
        else if point.x > self.right_limit() as f64 {
            self.left_limit() as f64
        }
        else {
            point.x
        };

        let y = if point.y < self.bottom_limit() as f64 {
            self.top_limit() as f64
        }
        else if point.y > self.top_limit() as f64 {
            self.bottom_limit() as f64
        }
        else {
            point.y
        };

        Point2::new(x, y)
    }
}
