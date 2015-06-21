use ncurses::*;
use self::Key::*;
use std::env;
use std::ptr::null_mut;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Key {
    Unknown,
    Escape,
    Up,
    Down,
    Left,
    Right,
}

pub fn get_key() -> Key {
    match getch() {
        104 | KEY_LEFT  => Left,
        106 | KEY_DOWN  => Down,
        107 | KEY_UP    => Up,
        108 | KEY_RIGHT => Right,

        27 | 113 => Escape,
        k => {
            printw(&format!("unknown key: {}", k)[..]);
            Unknown
        }
    }
}

pub fn setup() {
    env::set_var("ESCDELAY", "25");
    initscr();
    signal::setup();

    intrflush(null_mut(), false);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr, true);
    nonl(); // plain newlines
    cbreak(); // char-at-a-time input
}

pub fn teardown() {
    clear();
    refresh();
    endwin();
    println!("Thanks for playing.");
}

mod signal {
    use libc::c_int;
    use libc::consts::os::posix88::SIGINT;
    use libc::funcs::posix01::signal::signal;
    use libc::funcs::posix88::signal::kill;
    use libc::funcs::posix88::unistd::getpid;
    use ncurses::endwin;
    use std::mem::transmute;

    static SIG_DFL: u64 = 0;

    unsafe extern "C" fn interrupt(sig: c_int) {
        // don't handle again
        signal(sig, SIG_DFL);
        assert_eq!(sig, SIGINT);
        // clean up
        endwin();
        println!("Interrupted!");
        // signal again
        kill(getpid(), sig);
    }

    pub fn setup() {
        unsafe {
            signal(SIGINT, transmute(interrupt));
        }
    }
}
