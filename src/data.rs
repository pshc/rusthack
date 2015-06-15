#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Pos { pub x: u8, pub y: u8 }

impl Pos {
    pub fn new(x: u8, y: u8) -> Pos {
        Pos {x: x, y: y}
    }
}

pub struct Bounds {
    pub x: u8,
    pub y: u8,
    pub max_x: u8,
    pub max_y: u8,
}

impl Bounds {
    pub fn new(x: u8, y: u8, max_x: u8, max_y: u8) -> Bounds {
         Bounds {x: x, y: y, max_x: max_x, max_y: max_y }
    }
}


pub struct Level {
    pub player_pos: Pos,
    pub level_bounds: Bounds,
}

impl Level {
    pub fn new(player_pos: Pos, level_bounds: Bounds) -> Level {
        Level {
            player_pos: player_pos,
            level_bounds: level_bounds,
        }
    }
}
