use crate::model::{Editor, StatusBar, Terminal};
use std::fmt::Display;
use std::io::{self, Write};

impl Terminal {
    pub fn draw<T: Write>(
        &self,
        out: &mut T,
        editor: &mut Editor,
        statusbar: &mut StatusBar,
    ) -> Result<(), io::Error> {
        let str_vec: &mut Vec<String> = &mut vec![];

        editor.draw(str_vec).unwrap();
        statusbar.draw(str_vec, editor).unwrap();

        write!(out, "{}", &str_vec.concat())?;
        //out.write(&str_vec.concat().as_bytes()).unwrap();
        out.flush()?;
        return Ok(());
    }
}
pub enum TermDispType {
    Editor,
    StatusBar,
}

pub fn get_term_disp_size(disp_type: TermDispType) -> (usize, usize) {
    // Log::ep("★★　get_term_disp_size", "");
    let (cols, rows) = termion::terminal_size().unwrap();
    let (cols, rows) = (cols as usize, rows as usize);
    // Log::ep("cols", cols);
    //Log::ep("rows", rows);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Log {}

impl Log {
    pub fn ep<T: Display>(m: &str, v: T) {
        if cfg!(debug_assertions) {
            eprintln!("{} {}", format!("{m:?}", m = m), v);
        }
    }
    pub fn ep_s(m: &str) {
        if cfg!(debug_assertions) {
            eprintln!("{}", m);
        }
    }
}
