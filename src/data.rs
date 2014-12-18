#[deriving(Copy, Eq, PartialEq, Show)]
pub struct Pos { pub x: u8, pub y: u8 }

impl Pos {
    pub fn new(x: u8, y: u8) -> Pos {
        Pos {x: x, y: y}
    }
}

pub struct Level {
    pub player_pos: Pos,
}

impl Level {
    pub fn new() -> Level {
        Level {
            player_pos: Pos::new(0, 0),
        }
    }
}
