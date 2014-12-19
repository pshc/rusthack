
#[cfg(target_os = "macos")]
mod osx {
    use std::io::fs;
    use std::io::fs::PathExtensions;

    const TCOD_PATH: &'static str = "../libtcod";

    pub fn build() {
        // find paths
        let cwd = Path::new(file!()).dir_path();
        let tcod = cwd.join(TCOD_PATH);

        // copy libraries
        for entry in fs::readdir(&tcod).unwrap().iter() {
            if entry.is_file() {
                let filename = entry.filename_str().expect("couldn't get dylib filename");
                if filename.ends_with(".dylib") {
                    fs::copy(entry, &cwd.join(filename)).unwrap();
                }
            }
        }

        // copy resources
        fs::copy(&tcod.join("terminal.png"), &cwd.join("terminal.png")).unwrap();
    }
}

#[cfg(target_os = "macos")]
fn main() {
    osx::build();
}

#[cfg(not(target_os = "macos"))]
fn main() {
}
