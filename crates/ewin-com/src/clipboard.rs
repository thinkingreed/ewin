use crate::{global::*, log::*, model::*};
use anyhow::Context;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::{fs::OpenOptions, io::Read, io::Write, process, process::Command};
use subprocess::Exec;

pub fn set_clipboard(copy_string: &str) {
    Log::debug_s("set_clipboard ");
    if *ENV == Env::WSL {
        if *IS_POWERSHELL_ENABLE {
            let result = set_wsl_clipboard(copy_string);
            Log::debug("result", &result);
            if let Err(err) = result {
                Log::error("set_win_clipboard err", &err.to_string());
                let _ = CLIPBOARD.set(copy_string.to_string());
            }
        } else {
            let _ = CLIPBOARD.set(copy_string.to_string());
        }
    } else {
        let result: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
        match result {
            Ok(mut ctx) => ctx.set_contents(copy_string.to_string()).unwrap(),
            Err(err) => {
                Log::error("ClipboardProvider err", &err);
                let _ = CLIPBOARD.set(copy_string.to_string());
            }
        }
    };
}

fn set_wsl_clipboard(copy_str: &str) -> anyhow::Result<()> {
    let clipboard_file = FilePath::get_app_clipboard_file_path();
    // In the case of wsl, there is a length limit when passing a character string as a command argument, so file is used as an argument.
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&clipboard_file)?;
    file.write_all(copy_str.as_bytes())?;

    // clip.exe < file
    Exec::shell(format!("{}{}{}", "clip.exe", "<", clipboard_file.to_str().unwrap())).join()?;
    Ok(())
}

pub fn get_clipboard() -> anyhow::Result<String> {
    Log::debug_s("get_clipboard");
    if *ENV == Env::WSL {
        if *IS_POWERSHELL_ENABLE {
            get_wsl_clipboard()
        } else {
            Ok(CLIPBOARD.get().unwrap_or(&"".to_string()).clone())
        }
    } else {
        let provider: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
        match provider {
            Ok(mut ctx) => return Ok(ctx.get_contents().unwrap_or_else(|_| "".to_string())),
            Err(_) => {
                Log::debug_s("get memory");
                //       return Ok(self.clipboard.clone());
                return Ok(CLIPBOARD.get().unwrap_or(&"".to_string()).clone());
            }
        }
    }
}

fn get_wsl_clipboard() -> anyhow::Result<String> {
    let p = Command::new("powershell.exe").arg("get-clipboard").stdout(process::Stdio::piped()).spawn()?;
    let mut stdout = p.stdout.context("take stdout")?;
    let mut buf = String::new();
    stdout.read_to_string(&mut buf)?;

    // Remove new line(CRLF) for automatic insertion at the end
    buf = buf.chars().take(buf.chars().count() - 2).collect::<String>();

    Ok(buf)
}
