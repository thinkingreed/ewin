use crate::model::{Editor, Log};
use clipboard::{ClipboardContext, ClipboardProvider};

use anyhow::Context;
use std::io::Read;
use std::io::Write;
use std::process;
use std::process::Command;

impl Editor {
    pub fn set_clipboard(&mut self, copy_string: &str) -> anyhow::Result<()> {
        let mut p = Command::new("powershell.exe")
            .arg("set-clipboard")
            .arg("-Value")
            .arg(copy_string)
            .stdin(process::Stdio::piped())
            .spawn()
            .or_else(|_| Command::new("pbcopy").stdin(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("win32yank").arg("-i").stdin(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("win32yank.exe").arg("-i").stdin(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("xsel").arg("-bi").stdin(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("xclip").arg("-i").stdin(process::Stdio::piped()).spawn())?;
        {
            let mut stdin = p.stdin.take().context("take stdin")?;
            write!(stdin, "{}", copy_string)?;
        }
        p.wait()?;
        Ok(())
    }

    /*
    pub fn set_clipboard(&mut self, copy_string: String) {
        let result: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
        match result {
            Ok(mut ctx) => return ctx.set_contents(copy_string.clone()).unwrap(),
            Err(_) => return self.clipboard = copy_string.clone(),
        }
    }*/

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
            match result {
                Ok(mut ctx) => buf = ctx.get_contents().unwrap_or("".to_string()),
                Err(_) => buf = self.clipboard.clone(),
            }
            Log::ep("ClipboardContext", buf.clone());
        }
        Ok(buf)
    }
}
