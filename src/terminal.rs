use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Editor, Log, Prompt, StatusBar, Terminal};
use std::io::{self, Write};
use termion::cursor;

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, editor: &mut Editor, prompt: &mut Prompt, sbar: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("★　All draw");

        self.set_disp_size(editor, prompt, sbar);
        let str_vec: &mut Vec<String> = &mut vec![];

        editor.draw(str_vec).unwrap();

        prompt.draw(str_vec).unwrap();
        sbar.draw(str_vec, editor).unwrap();
        self.draw_cur(str_vec, editor, prompt);

        write!(out, "{}", &str_vec.concat())?;
        //out.write(&str_vec.concat().as_bytes()).unwrap();
        out.flush()?;
        return Ok(());
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor, prompt: &mut Prompt) {
        Log::ep_s("★  set_cur_str");
        Log::ep("cur.x", editor.cur.x);
        Log::ep("disp_x", editor.cur.disp_x);
        Log::ep("sel.sx", editor.sel.sx);
        Log::ep("sel.ex", editor.sel.ex);
        Log::ep("sel.s_disp_x", editor.sel.s_disp_x);
        Log::ep("sel.e_disp_x", editor.sel.e_disp_x);

        if prompt.is_save_new_file {
            Log::ep("prompt.cont.input.chars().count()", prompt.cont.buf.len());
            if prompt.cont.buf.len() == 0 {
                Log::ep_s("cursor::Goto");
                str_vec.push(cursor::Goto(1, (prompt.disp_row_posi + prompt.disp_row_num - 1) as u16).to_string());
            } else {
                str_vec.push(cursor::Goto(prompt.cont.cur.disp_x as u16, (prompt.disp_row_posi + prompt.disp_row_num - 1) as u16).to_string());
            }
        } else {
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

    pub fn set_disp_size(&mut self, editor: &mut Editor, prompt: &mut Prompt, sber: &mut StatusBar) {
        let (cols, rows) = termion::terminal_size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);

        if rows <= 10 {
            sber.disp_row_num = 0;
        } else {
            sber.disp_row_num = 1;
            sber.disp_row_posi = rows;
            sber.disp_col_num = cols;
        }
        prompt.disp_row_posi = rows - prompt.disp_row_num + 1 - sber.disp_row_num;
        editor.disp_row_num = rows - prompt.disp_row_num - sber.disp_row_num;
        editor.disp_col_num = cols;
    }
}
