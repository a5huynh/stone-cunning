use crate::Point3;

pub fn is_near(a: &Point3<i32>, b: &Point3<i32>) -> bool {
    let dist_x = (a.x - b.x).abs();
    let dist_y = (a.y - b.y).abs();

    dist_x + dist_y <= 1
}
