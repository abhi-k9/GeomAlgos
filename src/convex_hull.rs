use crate::{Line2D, Line3D, Point2D, Point3D};

fn get_convex_hull(point_cloud2D: &[Point2D]) {

}

fn get_convex_hull(point_cloud3D: &[Point3D]) {

}

#[cfg(test)]
mod tests {
    use super::*;

    fn square_convex_hull() {
        let point_cloud = vec![
            Point2D::new(-1, -1), Point2D::new(1, 1),
            Point2D::new(-1, 1), Point2D::new(1, -1),
            Point2D::new(0, 0)
        ];

        let correct_convex_hull = vec![
            Point2D::new(-1, -1), Point2D::new(-1, 1),
            Point2D::new(1, 1), Point2D::new(1, -1)
        ];

        assert_eq!(get_convex_hull(point_cloud), correct_convex_hull);
    }
}
