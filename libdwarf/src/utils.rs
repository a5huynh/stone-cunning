use crate::Point3;

pub fn is_near(a: &Point3<u32>, b: &Point3<u32>) -> bool {
    let dist_x = (a.x as i32 - b.x as i32).abs() as u32;
    let dist_y = (a.y as i32 - b.y as i32).abs() as u32;

    dist_x + dist_y <= 1
}
