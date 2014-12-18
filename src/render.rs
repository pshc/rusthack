use data::*;
use ncurses::*;

pub fn render(level: &Level) {
    put('@', level.player_pos);

    refresh();
}

fn put(c: char, pos: Pos) {
    mvaddch(pos.y as i32, pos.x as i32, c as u32);
}

