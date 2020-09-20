use crate::model::Editor;
use anyhow::Context;
use std::env;
use std::io::Read;
use std::process;
use std::process::Command;

impl Editor {
    pub fn clipboard_copy(&mut self, s: &str) -> anyhow::Result<()> {
        let PATH = env::var("PATH").unwrap();

        // Log::ep("PATH", PATH.clone());

        let mut p = Command::new("/usr/bin/echo")
            .arg(s)
            .arg("|")
            .arg("clip.exe")
            //       .env("PATH", PATH)
            // 環境変数を設定する
            .spawn()?;

        /*
        let mut p = Command::new("pbcopy")
            .stdin(process::Stdio::piped())
            .spawn()
            /*    .or_else(|_| {
                   Command::new(format!("{} {} {}", "echo", s, "| clip.exe"))
                       .stdin(process::Stdio::piped())
                       .spawn()
               })
            */
            .or_else(|_| {
                Command::new("win32yank")
                    .arg("-i")
                    .stdin(process::Stdio::piped())
                    .spawn()
            })
            .or_else(|_| {
                Command::new("win32yank.exe")
                    .arg("-i")
                    .stdin(process::Stdio::piped())
                    .spawn()
            })
            .or_else(|_| {
                Command::new("xsel")
                    .arg("-bi")
                    .stdin(process::Stdio::piped())
                    .spawn()
            })
            .or_else(|_| {
                Command::new("xclip")
                    .arg("-i")
                    .stdin(process::Stdio::piped())
                    .spawn()
            })?;*/

        p.wait()?;

        Ok(())
    }

    pub fn get_clipboard_paste(&mut self) -> anyhow::Result<String> {
        let p = Command::new("pbpaste")
            .stdout(process::Stdio::piped())
            .spawn()
            .or_else(|_| Command::new("win32yank").arg("-o").stdout(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("win32yank.exe").arg("-o").stdout(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("xsel").arg("-bo").stdout(process::Stdio::piped()).spawn())
            .or_else(|_| Command::new("xclip").arg("-o").stdout(process::Stdio::piped()).spawn())?;
        let mut stdout = p.stdout.context("take stdout")?;
        let mut buf = String::new();
        stdout.read_to_string(&mut buf)?;
        // win32yank.exe emits CRLF but I don't want to.
        buf = buf.replace('\r', "");
        Ok(buf)
    }
}
