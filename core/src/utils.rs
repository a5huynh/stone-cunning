use crate::WorldPos;

pub fn is_near(a: &WorldPos, b: &WorldPos) -> bool {
    let dist_x = (a.x - b.x).abs();
    let dist_y = (a.y - b.y).abs();

    dist_x + dist_y <= 1
}
