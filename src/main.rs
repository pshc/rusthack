#![feature(libc)]
extern crate libc;
extern crate tcod;

mod data;
mod io;
mod render;

fn step(level: &mut data::Level, io: &mut io::Io) -> bool {
    use io::Key::*;

    let key = io.get_key();
    if key.is_none() {
        return true;
    }

    match key.unwrap() {
        Escape => return false,
        Up => if level.player_pos.y > 0 { level.player_pos.y -= 1; },
        Down => level.player_pos.y += 1,
        Left => if level.player_pos.x > 0 { level.player_pos.x -= 1; },
        Right => level.player_pos.x += 1,
    }
    true
}

fn main() {
    let mut io = io::Io::new();

    let mut level = data::Level::new();
    while !io.window_closed() {
        render::render(&level, &mut io);
        if !step(&mut level, &mut io) {
            break;
        }
    }
}
