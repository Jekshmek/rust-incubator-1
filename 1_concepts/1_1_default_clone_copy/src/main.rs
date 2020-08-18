use crate::geometry::{Point, Polyline};

mod geometry {
    #[derive(Clone, Copy, Default, Debug)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Clone, Debug)]
    pub struct Polyline {
        pub points: Vec<Point>,
    }

    impl Polyline {
        pub fn new(point: Point) -> Polyline {
            Polyline {
                points: vec![point],
            }
        }

        pub fn from_vec(vec: Vec<Point>) -> Polyline {
            Polyline { points: vec }
        }

        pub fn len(&self) -> usize {
            self.points.len()
        }
    }
}

fn main() {
    println!("{:?}", Polyline::new(Point::default()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        assert_eq!(Polyline::new(Point::default()).points.len(), 1);
        assert_eq!(
            Polyline::from_vec(vec![Point::default(), Point::default()]).len(),
            2
        );
    }
}
