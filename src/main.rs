#![feature(globs)]
extern crate libc;
extern crate ncurses;

mod data;
mod io;
mod render;

fn step(level: &mut data::Level) -> bool {
    use io::Key::*;

    match io::get_key() {
        Escape => return false,
        Up => if level.player_pos.y > 0 { level.player_pos.y -= 1; },
        Down => level.player_pos.y += 1,
        Left => if level.player_pos.x > 0 { level.player_pos.x -= 1; },
        Right => level.player_pos.x += 1,
        Unknown => {}
    }
    true
}

fn main() {
    io::setup();

    let mut level = data::Level::new();
    loop {
        render::render(&level);
        if !step(&mut level) {
            break;
        }
    }

    io::teardown();
}
