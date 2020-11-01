use crate::_cfg::lang::cfg::LangCfg;
use crate::model::*;
use crate::model::{Editor, Log};
use std::io::{self, Write};
use termion::cursor;

use anyhow::Context;
use std::io::Read;
use std::process;
use std::process::Command;

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("★　All draw");

        self.set_disp_size(editor, mbar, prom, sbar);
        let str_vec: &mut Vec<String> = &mut vec![];

        let d_range = editor.d_range.get_range();
        if d_range.d_type != DType::Not {
            editor.set_textarea_color(str_vec);
            editor.draw(out);
            mbar.draw(out);

            prom.draw(str_vec);
            sbar.draw(str_vec, editor);
        }

        self.draw_cur(str_vec, editor, prom);

        write!(out, "{}", &str_vec.concat())?;
        out.flush()?;
        editor.d_range.clear();

        return Ok(());
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor, prom: &mut Prompt) {
        Log::ep_s("★  set_cur_str");
        Log::ep("cur.disp_x", editor.cur.disp_x);
        Log::ep("x_offset_disp", editor.x_offset_disp);

        if prom.is_save_new_file || prom.is_search || prom.is_replace {
            prom.draw_cur(str_vec);
        } else {
            //str_vec.push(cursor::Show.to_string());
            str_vec.push(cursor::Goto((editor.cur.disp_x - editor.x_offset_disp) as u16, (editor.cur.y + 1 - editor.y_offset) as u16).to_string());
        }
    }

    pub fn check_displayable(&mut self, lang_cfg: &LangCfg) -> bool {
        let (cols, rows) = termion::terminal_size().unwrap();
        if cols < 20 || rows < 8 {
            println!("{:?}", lang_cfg.terminal_size_small);
            return false;
        }
        return true;
    }

    pub fn set_disp_size(&mut self, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) {
        let (cols, rows) = termion::terminal_size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);

        Log::ep("rows", rows);
        Log::ep("cols", cols);

        if rows <= 10 {
            sbar.disp_row_num = 0;
        } else {
            sbar.disp_row_num = 1;
            sbar.disp_row_posi = rows;
            sbar.disp_col_num = cols;
        }
        prom.disp_row_posi = rows - prom.disp_row_num + 1 - sbar.disp_row_num;

        mbar.disp_col_num = cols;
        mbar.disp_row_posi = rows - prom.disp_row_num - sbar.disp_row_num;

        if mbar.msg_disp.len() > 0 {
            mbar.disp_row_num = 1;
        } else {
            mbar.disp_row_num = 0;
        }

        editor.disp_row_num = rows - prom.disp_row_num - mbar.disp_row_num - sbar.disp_row_num;
        // Log::ep("editor.disp_row_num", editor.disp_row_num);
        editor.disp_col_num = cols;
    }

    pub fn set_env(&mut self) {
        // WSL環境を判定出来ない為にpowershell試行
        let p = Command::new("uname").arg("-r").stdout(process::Stdio::piped()).spawn().unwrap();
        let mut stdout = p.stdout.context("take stdout").unwrap();
        let mut buf = String::new();
        stdout.read_to_string(&mut buf).unwrap();
        //   buf = buf.clone().trim().to_string();

        if buf.to_ascii_lowercase().contains("microsoft") {
            self.env = Env::WSL;
        } else {
            self.env = Env::Linux;
        }
    }
}
