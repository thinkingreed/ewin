use crate::global::*;
use anyhow::Context;
use ewin_cfg::log::*;
use ewin_const::models::env::*;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_env_platform() -> Env {
    let child = Command::new("uname").arg("-r").stdout(Stdio::piped()).spawn().unwrap();
    let mut stdout = child.stdout.context("take stdout").unwrap();
    let mut buf = String::new();
    stdout.read_to_string(&mut buf).unwrap();

    if buf.to_ascii_lowercase().contains("microsoft") {
        Env::WSL
    } else {
        Env::Linux
    }
}

#[cfg(target_os = "windows")]
pub fn get_env_platform() -> Env {
    return Env::Windows;
}

pub fn is_wsl_powershell_enable() -> bool {
    let mut rtn = false;
    if *ENV == Env::WSL {
        let result = Command::new("powershell.exe").stdout(Stdio::null()).stdin(Stdio::null()).stderr(Stdio::null()).spawn();
        rtn = result.is_ok();
    }
    rtn
}

pub fn change_output_encoding() {
    // If it is executed asynchronously, it will be reflected after the screen is drawn, and there will be a problem with the display
    // so wait for the end with synchronous execution.
    let result = Command::new("powershell.exe").arg("chcp 65001").stdout(Stdio::null()).stdin(Stdio::null()).stderr(Stdio::null()).output();
    Log::debug("change output encoding chcp 65001 ", &result.is_ok());
}
