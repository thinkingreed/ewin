use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Editor, Log, Prompt, StatusBar, Terminal};
use std::io::{self, Write};

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, editor: &mut Editor, prompt: &mut Prompt, sbar: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("★　All draw");

        self.set_disp_size(editor, prompt, sbar);
        let str_vec: &mut Vec<String> = &mut vec![];

        editor.draw(str_vec).unwrap();

        prompt.draw(str_vec).unwrap();
        sbar.draw(str_vec, editor).unwrap();
        editor.set_cur_str(str_vec, prompt);

        write!(out, "{}", &str_vec.concat())?;
        //out.write(&str_vec.concat().as_bytes()).unwrap();
        out.flush()?;
        return Ok(());
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
