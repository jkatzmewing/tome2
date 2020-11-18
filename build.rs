#[cfg(all(unix, not(target_os = "macos")))]
fn main() {

    // add unix dependencies below
    println!("cargo:rustc-flags=-l X11 -l curses -l jansson");
}

#[cfg(target_os = "macos")]
fn main() {

    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
