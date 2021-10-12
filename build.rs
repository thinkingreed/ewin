#[cfg(windows)]
extern crate windres;

#[cfg(windows)]
use windres::Build;

#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() {
    Build::new().compile("assets/icon/ewin.rc").unwrap();
}
