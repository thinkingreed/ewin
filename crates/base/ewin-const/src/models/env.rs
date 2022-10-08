#[derive(Debug, Eq, PartialEq)]
pub enum Env {
    WSL,
    Linux,
    Windows,
}

pub fn get_os_str() -> String {
    return if cfg!(windows) {
        "windows".to_string()
    } else if cfg!(macos) {
        "macos".to_string()
    } else {
        "linux".to_string()
    };
}
