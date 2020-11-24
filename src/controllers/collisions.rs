use crate::geometry::Point;

pub struct CollisionController;

impl CollisionController {
    pub fn player_to_wall_horizonal(pos1: &Point, pos2: &Point) -> Point {
        let mut move_point = Point::new(pos1.x, pos1.y);
        if pos2.x - 32 < pos1.x && pos1.x < pos2.x + 32 {
            if pos2.y - 16 < pos1.y && pos1.y < pos2.y + 16 {
                let diff_x = pos1.x - pos2.x;
                move_point.x = pos2.x + 32 * (diff_x / diff_x.abs());
            }
        }
        return move_point;
    }

    pub fn player_to_wall_vertical(pos1: &Point, pos2: &Point) -> Point {
        let mut move_point = Point::new(pos1.x, pos1.y);
        if pos2.y - 32 < pos1.y && pos1.y < pos2.y + 32 {
            if pos2.x - 16 < pos1.x && pos1.x < pos2.x + 16 {
                let diff_y = pos1.y - pos2.y;
                move_point.y = pos2.y + 32 * (diff_y / diff_y.abs());
            }
        }
        return move_point;
    }

    pub fn player_to_wall_corner(pos1: &Point, pos2: &Point) -> Point {
        let mut move_point = Point::new(pos1.x, pos1.y);
        let diff_x1 = pos1.x - (pos2.x - 16);
        let diff_x2 = pos1.x - (pos2.x + 16);
        let diff_y1 = pos1.y - (pos2.y - 16);
        let diff_y2 = pos1.y - (pos2.y + 16);
        let dist11 = diff_x1 * diff_x1 + diff_y1 * diff_y1;
        let dist12 = diff_x1 * diff_x1 + diff_y2 * diff_y2;
        let dist21 = diff_x2 * diff_x2 + diff_y1 * diff_y1;
        let dist22 = diff_x2 * diff_x2 + diff_y2 * diff_y2;
        if dist11 < 256 {
            move_point.x = pos1.x + diff_x1 * (16 - (dist11 as f64).sqrt() as i32) / 16;
            move_point.y = pos1.y + diff_y1 * (16 - (dist11 as f64).sqrt() as i32) / 16;
            return move_point;
        }
        if dist12 < 256 {
            move_point.x = pos1.x + diff_x1 * (16 - (dist12 as f64).sqrt() as i32) / 16;
            move_point.y = pos1.y + diff_y2 * (16 - (dist12 as f64).sqrt() as i32) / 16;
            return move_point;
        }
        if dist21 < 256 {
            move_point.x = pos1.x + diff_x2 * (16 - (dist21 as f64).sqrt() as i32) / 16;
            move_point.y = pos1.y + diff_y1 * (16 - (dist21 as f64).sqrt() as i32) / 16;
            return move_point;
        }
        if dist22 < 256 {
            move_point.x = pos1.x + diff_x2 * (16 - (dist22 as f64).sqrt() as i32) / 16;
            move_point.y = pos1.y + diff_y2 * (16 - (dist22 as f64).sqrt() as i32) / 16;
            return move_point;
        }

        return move_point;
    }

    pub fn player_to_fire(pos1: &Point, pos2: &Point) -> bool {
        if pos2.x - 16 < pos1.x && pos1.x < pos2.x + 16 {
            if pos2.y - 16 < pos1.y && pos1.y < pos2.y + 16 {
                return true;
            }
        }
        return false;
    }
}
