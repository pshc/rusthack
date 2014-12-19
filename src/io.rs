use self::Key::*;
use tcod;

#[deriving(Eq, PartialEq, Show)]
pub enum Key {
    Unknown,
    Escape,
    Up,
    Down,
    Left,
    Right,
}

fn map_key(key: tcod::Key) -> Key {
    use tcod::Key::{Printable, Special};
    use tcod::KeyCode;

    match key {
        Special(KeyCode::Up) | Printable('k') => Up,
        Special(KeyCode::Down) | Printable('j') => Down,
        Special(KeyCode::Left) | Printable('h') => Left,
        Special(KeyCode::Right) | Printable('l') => Right,
        Special(KeyCode::Escape) | Printable('q') => Escape,
        _ => Unknown
    }
}

pub struct Io {
    pub con: tcod::Console,
}

impl Io {
    pub fn new() -> Io {
        Io {
            con: tcod::Console::init_root(80, 50, "HACK", false),
        }
    }

    pub fn get_key(&mut self) -> Key {
        let k = tcod::Console::wait_for_keypress(true);
        if k.pressed {
            map_key(k.key)
        }
        else {
            // was a keyup event; try one more time
            let k = tcod::Console::wait_for_keypress(true);
            assert!(k.pressed, "I need a damn key already");
            map_key(k.key)
        }
    }

    pub fn window_closed(&self) -> bool {
        tcod::Console::window_closed()
    }
}
