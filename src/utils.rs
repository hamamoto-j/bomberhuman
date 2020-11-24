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
    Point::new(32 + 32 * (i as i32 % 15), 32 + 32 * (i as i32 / 15))
}

pub fn pos_to_idx(pos: Point) -> usize {
    (((pos.x - 16) / 32) + 15 * ((pos.y - 16) / 32)) as usize
}

pub fn is_eq_pos(pos1: Point, pos2: Point) -> bool {
    if pos1.x == pos2.x {
        if pos1.y == pos2.y {
            return true;
        }
    }
    return false;
}

pub fn players_pos(i: i32) -> Point {
    match i {
        0 => return Point::new(64, 64),
        1 => return Point::new(32 + 32 * 13, 32 + 32 * 11),
        2 => return Point::new(32 + 32 * 13, 64),
        3 => return Point::new(64, 32 + 32 * 11),
        _ => return Point::new(64, 64),
    }
}
