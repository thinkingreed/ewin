#[cfg(windows)]
extern crate windres;

#[cfg(windows)]
use windres::Build;

#[cfg(unix)]
fn main() {}

#[cfg(windows)]
fn main() {
    Build::new().compile("../assets/icon/ewin.rc").unwrap();
}
