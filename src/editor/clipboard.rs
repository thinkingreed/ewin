use crate::model::{Editor, Log};
use clipboard::{ClipboardContext, ClipboardProvider};

use anyhow::Context;
use std::io::Read;
use std::io::Write;
use std::process;
use std::process::Command;

impl Editor {
    pub fn set_clipboard(&mut self, copy_string: &str) {
        if let Err(err) = self.try_set_clipboard(&copy_string) {
            Log::ep("try_set_clipboard err", err.to_string());
            let result: Result<ClipboardContext, Box<_>> = ClipboardProvider::new();
            match result {
                Ok(mut ctx) => ctx.set_contents(copy_string.to_string()).unwrap(),
                Err(_) => {
                    Log::ep_s("set memory");
                    self.clipboard = copy_string.to_string();
                }
            }
        }
    }
    fn try_set_clipboard(&mut self, copy_string: &str) -> anyhow::Result<()> {
        // WSL環境を判定出来ない為にpowershell試行
        let mut p = Command::new("powershell.exe").arg("set-clipboard").arg("-Value").arg(copy_string).stdin(process::Stdio::piped()).spawn()?;
        {
            let mut stdin = p.stdin.take().context("take stdin")?;
            write!(stdin, "{}", copy_string)?;
        }
        p.wait()?;
        Ok(())
    }

    pub fn get_clipboard(&mut self) -> anyhow::Result<String> {
        let result = self.try_get_clipboard();

        match result {
            Ok(string) => return Ok(string),
            Err(err) => {
                Log::ep("try_get_clipboard err", err.to_string());
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
    }

    fn try_get_clipboard(&mut self) -> anyhow::Result<String> {
        // WSL環境を判定出来ない為にpowershell試行
        let p = Command::new("powershell.exe").arg("get-clipboard").stdout(process::Stdio::piped()).spawn()?;
        let mut stdout = p.stdout.context("take stdout")?;
        let mut buf = String::new();
        stdout.read_to_string(&mut buf)?;
        buf = buf.clone().trim().to_string();

        Ok(buf)
    }
}
