pub fn cart2iso(x: f32, y: f32) -> (f32, f32) {
    (
        x - y,
        (x + y) / 2.0,
    )
}

pub fn iso2cart(x: f32, y: f32) -> (f32, f32) {
    (
        (2.0 * y + x) / 2.0,
        (2.0 * y - x) / 2.0
    )
}