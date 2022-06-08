use crate::{global::*, model::*};
use anyhow::Context;
use clipboard::{ClipboardContext, ClipboardProvider};
use ewin_cfg::{lang::lang_cfg::Lang, log::Log, model::modal::CFgFilePath};
use ewin_const::{def::NEW_LINE_LF, model::Env};
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
    let clipboard_file = CFgFilePath::get_app_clipboard_file_path();
    // In the case of wsl, there is a length limit when passing a character string as a command argument, so file is used as an argument.
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&clipboard_file)?;
    file.write_all(copy_str.as_bytes())?;

    // When there is only one character per line in the WSL environment
    // the copy of the character string fails due to a bug in clip.exe.
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

pub fn check_clipboard(is_prompt: bool) -> ActType {
    let clipboard = get_clipboard().unwrap_or_else(|_| "".to_string());

    if clipboard.is_empty() {
        return ActType::Draw(DParts::MsgBar(Lang::get().no_value_in_clipboard.to_string()));
    }
    // Do not paste multiple lines for Prompt
    if is_prompt {
        // Check multiple lines
        if clipboard.match_indices(&NEW_LINE_LF.to_string()).count() > 0 {
            return ActType::Draw(DParts::MsgBar(Lang::get().cannot_paste_multi_rows.to_string()));
        };
    }
    return ActType::Next;
}
