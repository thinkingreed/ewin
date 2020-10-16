use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Editor, Log, MsgBar, Prompt, StatusBar, Terminal};
use std::io::{self, Write};
use termion::cursor;

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("★　All draw");

        self.set_disp_size(editor, mbar, prom, sbar);
        let str_vec: &mut Vec<String> = &mut vec![];

        editor.draw(str_vec);
        mbar.draw(str_vec);

        prom.draw(str_vec);
        sbar.draw(str_vec, editor);
        self.draw_cur(str_vec, editor, prom);

        write!(out, "{}", &str_vec.concat())?;
        out.flush()?;
        return Ok(());
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor, prom: &mut Prompt) {
        Log::ep_s("★  set_cur_str");

        if prom.is_save_new_file || prom.is_search || prom.is_replace {
            prom.draw_cur(str_vec);
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

    pub fn set_disp_size(&mut self, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) {
        let (cols, rows) = termion::terminal_size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);

        Log::ep("rows", rows);

        if rows <= 10 {
            sbar.disp_row_num = 0;
        } else {
            sbar.disp_row_num = 1;
            sbar.disp_row_posi = rows;
            sbar.disp_col_num = cols;
        }
        // Log::ep("sbar.disp_row_num", sbar.disp_row_num);
        // Log::ep("prom.disp_row_num", prom.disp_row_num);
        prom.disp_row_posi = rows - prom.disp_row_num + 1 - sbar.disp_row_num;
        mbar.disp_col_num = cols;

        mbar.disp_row_posi = rows - prom.disp_row_num - sbar.disp_row_num;
        // Log::ep("prom.disp_row_posi", prom.disp_row_posi);
        // Log::ep("mbar.disp_col_num", mbar.disp_col_num);
        // Log::ep(" mbar.disp_row_posi", mbar.disp_row_posi);

        if mbar.msg_disp.len() > 0 {
            mbar.disp_row_num = 1;
        } else {
            mbar.disp_row_num = 0;
        }

        editor.disp_row_num = rows - prom.disp_row_num - mbar.disp_row_num - sbar.disp_row_num;

        Log::ep("editor.disp_row_num", editor.disp_row_num);

        editor.disp_col_num = cols;
    }
}
