
#[cfg(target_os = "macos")]
mod osx {
    use std::io::fs::copy;

    const TCOD_PATH: &'static str = "../libtcod";

    pub fn build() {
        // find paths
        let cwd = Path::new(file!()).dir_path();
        let tcod = cwd.join(TCOD_PATH);

        // provide dylib path
        println!("cargo:rustc-flags=-L {}", tcod.display());

        // copy resources
        copy(&tcod.join("terminal.png"), &cwd.join("terminal.png")).unwrap();
    }
}

#[cfg(target_os = "macos")]
fn main() {
    osx::build();
}

#[cfg(not(target_os = "macos"))]
fn main() {
}
