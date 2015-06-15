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
        Up => if level.player_pos.y > level.level_bounds.y { level.player_pos.y -= 1; },
        Down => if level.player_pos.y < level.level_bounds.max_y { level.player_pos.y += 1 },
        Left => if level.player_pos.x > level.level_bounds.x { level.player_pos.x -= 1; },
        Right => if level.player_pos.x < level.level_bounds.max_x { level.player_pos.x += 1 },
    }
    true
}

fn main() {
    let mut io = io::Io::new();

    let mut level = data::Level::new(
        data::Pos::new(0,0),
        data::Bounds::new(0,0,79,49),
        );
    while !io.window_closed() {
        render::render(&level, &mut io);
        if !step(&mut level, &mut io) {
            break;
        }
    }
}
