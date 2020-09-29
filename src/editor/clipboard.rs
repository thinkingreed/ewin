use crate::model::{Editor, Log};
use clipboard::{ClipboardContext, ClipboardProvider};

use anyhow::Context;
use std::io::Read;
use std::io::Write;
use std::process;
use std::process::Command;

impl Editor {
    pub fn set_clipboard(&mut self, copy_string: &str) {
        if let Err(err) = self.try_clipboard(&copy_string) {
            Log::ep("try_clipboard err", err.to_string());
            let result: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
            match result {
                Ok(mut ctx) => ctx.set_contents(copy_string.to_string()).unwrap(),
                Err(_) => {
                    Log::ep_s("set memory");
                    self.clipboard = copy_string.to_string();
                }
            }
        }

        /*
        match self.try_clipboard(&copy_string) {
            Ok(_) => {}
            Err(err) => {
                Log::ep("try_clipboard err", err.to_string());
                let result: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
                match result {
                    Ok(mut ctx) => ctx.set_contents(copy_string.to_string()).unwrap(),
                    Err(_) => {
                        Log::ep_s("set memory");
                        self.clipboard = copy_string.to_string();
                    }
                }
            }
        }*/
    }
    pub fn try_clipboard(&mut self, copy_string: &str) -> anyhow::Result<()> {
        let mut p = Command::new("powershell.exe").arg("set-clipboard").arg("-Value").arg(copy_string).stdin(process::Stdio::piped()).spawn()?;
        {
            let mut stdin = p.stdin.take().context("take stdin")?;
            write!(stdin, "{}", copy_string)?;
        }
        p.wait()?;
        Ok(())
    }

    pub fn get_clipboard(&mut self) -> anyhow::Result<String> {
        let p = Command::new("powershell.exe")
            .arg("get-clipboard")
            .stdout(process::Stdio::piped())
            .spawn()
            .or_else(|_| Command::new("pbpaste").stdout(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("win32yank").arg("-o").stdout(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("win32yank.exe").arg("-o").stdout(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("xsel").arg("-bo").stdout(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("xclip").arg("-o").stdout(process::Stdio::piped()).spawn())?;
        let mut stdout = p.stdout.context("take stdout")?;
        let mut buf = String::new();
        stdout.read_to_string(&mut buf)?;

        buf = buf.clone().trim().to_string();

        if buf.len() == 0 {
            let result: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
            if let Ok(mut ctx) = result {
                buf = ctx.get_contents().unwrap_or("".to_string())
            }
            Log::ep("ClipboardContext", buf.clone());
        }
        Ok(buf)
    }
}
