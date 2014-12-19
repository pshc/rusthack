use data::*;
use tcod::{BackgroundFlag, Console};
use io::Io;

pub fn render(level: &Level, io: &mut Io) {
    let con = &mut io.con;

    con.clear();
    put('@', level.player_pos, con);

    Console::flush();
}

fn put(c: char, pos: Pos, con: &mut Console) {
    con.put_char(pos.x as int, pos.y as int, c, BackgroundFlag::Set);
}

