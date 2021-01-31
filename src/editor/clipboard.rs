use crate::{def::*, global::*, model::*};
use anyhow::Context;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::Read;
use std::io::Write;
use std::process;
use std::process::Command;

impl Core {
    pub fn set_clipboard(&mut self, copy_string: &str) {
        if *ENV == Env::WSL {
            Log::ep_s("try_set_clipboard ");
            if let Err(err) = self.try_set_clipboard(&copy_string) {
                Log::ep("try_set_clipboard err", &err.to_string());
            }
        } else {
            Log::ep_s("ClipboardProvider::new() ");
            let result: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
            match result {
                Ok(mut ctx) => ctx.set_contents(copy_string.to_string()).unwrap(),
                Err(err) => {
                    Log::ep("ClipboardProvider err", &err);
                    self.clipboard = copy_string.to_string();
                }
            }
        };
    }
    fn try_set_clipboard(&mut self, copy_string: &str) -> anyhow::Result<()> {
        let mut p = Command::new("powershell.exe").arg("set-clipboard").arg("-Value").arg(copy_string).stdin(process::Stdio::piped()).spawn()?;
        // let mut p = Command::new("echo").arg("off").arg(copy_string).arg("|").arg("clip.exe").stdin(process::Stdio::piped()).spawn()?;
        {
            let mut stdin = p.stdin.take().context("take stdin")?;
            write!(stdin, "{}", copy_string)?;
        }
        p.wait()?;
        Ok(())
    }

    pub fn get_clipboard(&mut self) -> anyhow::Result<String> {
        if *ENV == Env::WSL {
            Log::ep_s("try_get_clipboard");
            return self.try_get_clipboard();
        } else {
            Log::ep_s("ClipboardProvider::new() ");
            let provider: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
            match provider {
                Ok(mut ctx) => return Ok(ctx.get_contents().unwrap_or("".to_string())),
                Err(_) => {
                    Log::ep_s("get memory");
                    return Ok(self.clipboard.clone());
                }
            }
        }
    }

    fn try_get_clipboard(&mut self) -> anyhow::Result<String> {
        let p = Command::new("powershell.exe").arg("get-clipboard").stdout(process::Stdio::piped()).spawn()?;
        let mut stdout = p.stdout.context("take stdout")?;
        let mut buf = String::new();
        stdout.read_to_string(&mut buf)?;

        // Windowsからのpasteで\r\n対応
        let mut buf = buf.replace(NEW_LINE_CRLF, NEW_LINE.to_string().as_str());
        Log::ep("buf ", &buf);
        // 末尾の自動挿入の改行の削除
        buf = buf.chars().take(buf.chars().count() - 1).collect::<String>();

        Ok(buf)
    }
}
