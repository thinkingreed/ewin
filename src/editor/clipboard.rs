use crate::{def::*, global::*, log::*, model::*};
use anyhow::Context;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::Read;
use std::io::Write;
use std::process;
use std::process::Command;

impl Editor {
    pub fn set_clipboard(&mut self, copy_string: &str) {
        Log::debug_s("set_win_clipboard ");
        if *ENV == Env::WSL {
            if *IS_POWERSHELL_ENABLE {
                if let Err(err) = self.set_win_clipboard(&copy_string) {
                    Log::error("set_win_clipboard err", &err.to_string());
                    self.clipboard = copy_string.to_string();
                }
            } else {
                self.clipboard = copy_string.to_string();
            }
        } else {
            let result: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
            match result {
                Ok(mut ctx) => ctx.set_contents(copy_string.to_string()).unwrap(),
                Err(err) => {
                    Log::error("ClipboardProvider err", &err);
                    self.clipboard = copy_string.to_string();
                }
            }
        };
        Log::debug("self.clipboard", &self.clipboard);
    }
    fn set_win_clipboard(&mut self, copy_string: &str) -> anyhow::Result<()> {
        let mut p = Command::new("powershell.exe").arg("set-clipboard").arg("-Value").arg(copy_string).stdin(process::Stdio::piped()).spawn()?;
        {
            let mut stdin = p.stdin.take().context("take stdin")?;
            write!(stdin, "{}", copy_string)?;
        }
        p.wait()?;
        Ok(())
    }

    pub fn get_clipboard(&mut self) -> anyhow::Result<String> {
        Log::debug_s("get_win_clipboard");
        if *ENV == Env::WSL {
            if *IS_POWERSHELL_ENABLE {
                return self.get_win_clipboard();
            } else {
                return Ok(self.clipboard.clone());
            }
        } else {
            let provider: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
            match provider {
                Ok(mut ctx) => return Ok(ctx.get_contents().unwrap_or("".to_string())),
                Err(_) => {
                    Log::debug_s("get memory");
                    return Ok(self.clipboard.clone());
                }
            }
        }
    }

    fn get_win_clipboard(&mut self) -> anyhow::Result<String> {
        let p = Command::new("powershell.exe").arg("get-clipboard").stdout(process::Stdio::piped()).spawn()?;
        let mut stdout = p.stdout.context("take stdout")?;
        let mut buf = String::new();
        stdout.read_to_string(&mut buf)?;

        // Windowsからのpasteで\r\n対応
        let mut buf = buf.replace(NEW_LINE_CRLF, NEW_LINE_LF.to_string().as_str());
        // Remove new line for automatic insertion at the end
        buf = buf.chars().take(buf.chars().count() - 1).collect::<String>();

        Ok(buf)
    }
}
