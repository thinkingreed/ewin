use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Editor, Log, Prompt, StatusBar, Terminal};
use std::io::{self, Write};

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, editor: &mut Editor, prompt: &mut Prompt, sbar: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("★　All draw");

        Terminal::set_disp_size(editor, prompt, sbar);
        let str_vec: &mut Vec<String> = &mut vec![];

        editor.draw(str_vec).unwrap();
        prompt.draw(str_vec, editor).unwrap();
        sbar.draw(str_vec, editor).unwrap();
        editor.set_cur_str(str_vec);

        write!(out, "{}", &str_vec.concat())?;
        //out.write(&str_vec.concat().as_bytes()).unwrap();
        out.flush()?;
        return Ok(());
    }

    pub fn check_displayable(lang_cfg: &LangCfg) -> bool {
        let (cols, rows) = termion::terminal_size().unwrap();
        if cols < 20 || rows < 8 {
            println!("{:?}", lang_cfg.terminal_size_small);
            return false;
        }
        return true;
    }

    pub fn set_disp_size(editor: &mut Editor, prompt: &mut Prompt, sber: &mut StatusBar) {
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
    pub fn get_term_disp_siz(disp_type: TermDispType) -> (usize, usize) {
        let (cols, rows) = termion::terminal_size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);

        match disp_type {
            TermDispType::StatusBar => {
                let mut status_bar_rows = rows;
                if rows < StatusBar::MIN_NUM_LINES_TO_DISP {
                    // 非表示
                    status_bar_rows = 0;
                }
                return (status_bar_rows, cols);
            }
            TermDispType::Editor => {
                let mut not_editor_lines = 0;
                if rows > StatusBar::MIN_NUM_LINES_TO_DISP {
                    not_editor_lines += 1;
                }
                return (rows - not_editor_lines, cols);
            }
        }
    }
}
pub enum TermDispType {
    Editor,
    StatusBar,
}
