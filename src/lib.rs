use std::fmt::Debug;

pub mod geometry_elements {

    #[derive(Debug, PartialEq, Default)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl Point2D {
        fn new(x: f64, y: f64) -> Point2D {
            Point2D{x, y}
        }
    }

    #[derive(Debug, PartialEq, Default)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64
    }

    impl Point3D {
        fn new(x: f64, y: f64, z: f64) -> Point3D {
            Point3D{x, y, z}
        }
    }

    #[derive(Debug, Default)]
    struct Segment2D {
        start: Point2D,
        end: Point2D
    }

    impl Segment2D {
        fn new(start: Point2D, end: Point2D) -> Segment2D {
            Segment2D{start, end}
        }
    }

    #[derive(Debug, Default)]
    struct Segment3D {
        start: Point3D,
        end: Point3D
    }
    
    impl Segment3D {
        fn new(start: Point3D, end: Point3D) -> Segment3D {
            Segment3D{start, end}
        }
    }

    #[derive(Debug, Default)]
    struct Line2D {
        line: Vector2D
    }

    impl Line2D {
        fn new(line: Line2D) -> Line2D {
            Line2D{line}
        }
    }

    #[derive(Debug, Default)]
    struct Line3D {
        line: Vector3D
    }
    
    impl Line3D {
        fn new(line: Line3D) -> Line3D {
            Line3D{line}
        }
    }

    #[derive(Debug, Default)]
    struct Vector2D {
        start: Point2D,
        end: Point2D
    }

    impl Vector2D {
        fn new(start: Point3D, end: Point3D) -> Vector2D {
            Vector2D{start, end}
        }
    }

    #[derive(Debug, Default)]
    struct Vector3D {
        start: Point3D,
        end: Point3D
    }

    impl Vector3D {
        fn new(start: Point3D, end: Point3D) -> Vector3D {
            Vector3D{start, end}
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
