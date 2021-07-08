use crate::def::{NEW_LINE_CRLF, NEW_LINE_LF};
use crate::{global::*, log::*, model::*};
use anyhow::Context;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::Read;
use std::io::Write;
use std::iter::FromIterator;
use std::process;
use std::process::Command;

pub fn set_clipboard(copy_string: &String) {
    Log::debug_s("set_clipboard ");
    if *ENV == Env::WSL {
        if *IS_POWERSHELL_ENABLE {
            if let Err(err) = set_win_clipboard(copy_string) {
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
fn set_win_clipboard(copy_string: &String) -> anyhow::Result<()> {
    Log::debug("copy_string", &copy_string);

    let escape_string = get_wsl_str(&copy_string);

    Log::debug("escape_string", &escape_string);

    let mut p = Command::new("powershell.exe").arg("set-clipboard").arg("-Value").arg(&escape_string).stdin(process::Stdio::piped()).spawn()?;
    {
        let mut stdin = p.stdin.take().context("take stdin")?;
        write!(stdin, "{}", &escape_string)?;
    }
    p.wait()?;
    Ok(())
}
// WSL:powershell.clipboard
// enclose the string in "’ "
// new line are ","
// Empty line is an empty string
fn get_wsl_str(str: &String) -> String {
    let mut copy_str: String = String::new();

    // TODO nl
    let str = str.replace(NEW_LINE_CRLF, &NEW_LINE_LF.to_string());
    let vec = Vec::from_iter(str.split(NEW_LINE_LF).map(String::from));
    for (i, str) in vec.iter().enumerate() {
        let tmp_vec: Vec<char> = str.chars().collect();
        let mut tmp_str = String::new();
        for c in tmp_vec {
            match c {
                '\'' => tmp_str.push_str("''"),
                _ => tmp_str.push(c),
            }
        }
        let ss = if tmp_str == "" { "''".to_string() } else { format!("'{}'", tmp_str) };
        copy_str.push_str(ss.as_str());
        if i != vec.len() - 1 {
            copy_str.push_str(",");
        }
    }
    return copy_str;
}

pub fn get_clipboard() -> anyhow::Result<String> {
    Log::debug_s("get_win_clipboard");
    if *ENV == Env::WSL {
        if *IS_POWERSHELL_ENABLE {
            return get_win_clipboard();
        } else {
            return Ok(CLIPBOARD.get().unwrap_or(&"".to_string()).clone());
        }
    } else {
        let provider: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
        match provider {
            Ok(mut ctx) => return Ok(ctx.get_contents().unwrap_or("".to_string())),
            Err(_) => {
                Log::debug_s("get memory");
                //       return Ok(self.clipboard.clone());
                return Ok(CLIPBOARD.get().unwrap_or(&"".to_string()).clone());
            }
        }
    }
}

fn get_win_clipboard() -> anyhow::Result<String> {
    let p = Command::new("powershell.exe").arg("get-clipboard").stdout(process::Stdio::piped()).spawn()?;
    let mut stdout = p.stdout.context("take stdout")?;
    let mut buf = String::new();
    stdout.read_to_string(&mut buf)?;

    Log::debug_s("buf");

    // Remove new line(CRLF) for automatic insertion at the end
    buf = buf.chars().take(buf.chars().count() - 2).collect::<String>();

    Ok(buf)
}
