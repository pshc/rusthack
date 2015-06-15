use data::*;
use tcod::{BackgroundFlag, Console, RootConsole};
use io::Io;

pub fn render(level: &Level, io: &mut Io) {
    let con = &mut io.con;

    con.clear();
    put('@', &level.player_pos, con);

    con.flush();
}

fn put(c: char, pos: &Pos, con: &mut RootConsole) {
    con.put_char(pos.x as i32, pos.y as i32, c, BackgroundFlag::Set);
}

