use self::Key::*;
use tcod;

#[derive(Eq, PartialEq, Debug)]
pub enum Key {
    Escape,
    Up,
    Down,
    Left,
    Right,
}

fn map_key(key: tcod::input::Key) -> Option<Key> {
    use tcod::input::Key::{Printable, Special};
    use tcod::input::KeyCode;

    Some(match key {
        Special(KeyCode::Up) | Printable('k') => Up,
        Special(KeyCode::Down) | Printable('j') => Down,
        Special(KeyCode::Left) | Printable('h') => Left,
        Special(KeyCode::Right) | Printable('l') => Right,
        Special(KeyCode::Escape) | Printable('q') => Escape,
        _ => { return None; }
    })
}

pub struct Io {
    pub con: tcod::RootConsole,
}

impl Io {
    pub fn new() -> Io {
        Io {
            con: tcod::RootConsole::initializer()
                .size(80, 50)
                .title("HACK")
                .init(),
        }
    }

    pub fn get_key(&mut self) -> Option<Key> {
        let k = self.con.wait_for_keypress(true);
        if k.pressed {
            map_key(k.key)
        }
        else if self.con.window_closed() {
            None
        }
        else {
            // was a keyup event; try one more time
            let k = self.con.wait_for_keypress(true);
            if k.pressed {
                map_key(k.key)
            }
            else {
                None
            }
        }
    }

    pub fn window_closed(&self) -> bool {
        self.con.window_closed()
    }
}
