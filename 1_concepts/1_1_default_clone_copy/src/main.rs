#[derive(Clone, Copy, Default, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Polyline {
    pub points: Vec<Point>,
}

impl Polyline {
    pub fn new(point: Point) -> Polyline {
        Polyline {
            points: vec![point],
        }
    }

    pub fn from_vec(vec: Vec<Point>) -> Polyline {
        Polyline {
            points: vec,
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
        assert_eq!(Polyline::from_vec(vec![Point::default(), Point::default()]).points.len(), 2);
    }
}
