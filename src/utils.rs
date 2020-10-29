use crate::geometry::Point;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


pub fn idx_to_pos(i: usize) -> Point {
    Point::new(32 + 32 * (i as i32 % 13), 32 + 32 * (i as i32 / 13))
}

pub fn pos_to_idx(pos: Point) -> usize {
   (((pos.x - 16) / 32) + 13 * ((pos.y - 16) /32)) as usize
}