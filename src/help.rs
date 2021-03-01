use crate::{colors::*, def::*, global::*, log::*, model::*, msgbar::*, prompt::prompt::*, statusbar::StatusBar, terminal::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::env;

#[derive(Debug, Clone)]
pub struct Help {
    pub mode: HelpMode,
    // Number displayed on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub key_bind_vec: Vec<Vec<KeyBind>>,
}
impl Default for Help {
    fn default() -> Self {
        Help {
            mode: HelpMode::None,
            disp_row_num: 0,
            disp_row_posi: 0,
            key_bind_vec: vec![],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// HelpMode
pub enum HelpMode {
    Show,
    // Details,
    None,
}
#[derive(Debug, Clone)]
pub struct KeyBind {
    pub key: String,
    pub func: String,
}

impl Help {
    pub const DISP_ROW_NUM: usize = 5;

    const KEY_WIDTH: usize = 9;
    const KEY_WIDTH_WIDE: usize = 11;
    const KEY_FUNC_WIDTH: usize = 8;
    const KEY_FUNC_WIDTH_WIDE: usize = 12;

    pub fn new() -> Self {
        return Help { ..Help::default() };
    }

    pub fn disp_toggle(&mut self, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) {
        self.mode = match self.mode {
            HelpMode::Show => HelpMode::None,
            HelpMode::None => HelpMode::Show,
        };

        if self.mode == HelpMode::Show {
            Terminal::set_disp_size(editor, mbar, prom, self, sbar);
            // Cursor moves out of help display area
            if editor.cur.y - editor.offset_y > editor.disp_row_num - 1 {
                editor.cur.y = editor.offset_y + editor.disp_row_num - 1;
                editor.cur.x = editor.rnw;
                editor.cur.disp_x = editor.rnw + 1;
            }
            editor.d_range.draw_type = DrawType::Not;
        } else {
            editor.d_range = DRange::new(editor.disp_row_num - 1, 0, DrawType::After);
        }
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("　　　　　　　　Help.draw");

        if self.mode == HelpMode::None {
            return;
        }
        if self.key_bind_vec.is_empty() {
            let mut vec: Vec<KeyBind> = vec![];

            // 1st line
            self.set_key_bind(&mut vec, KEY_CLOSE, &LANG.end);
            self.set_key_bind(&mut vec, KEY_SAVE, &LANG.save);
            self.set_key_bind_wide(&mut vec, KEY_COPY, &LANG.copy);
            self.set_key_bind_wide(&mut vec, KEY_PASTE, &LANG.paste);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 2nd line
            self.set_key_bind(&mut vec, KEY_UNDO, &LANG.undo);
            self.set_key_bind(&mut vec, KEY_REDO, &LANG.redo);
            self.set_key_bind_wide(&mut vec, KEY_SEARCH, &LANG.search);
            self.set_key_bind_wide(&mut vec, KEY_REPLACE, &LANG.replace);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 3rd line
            self.set_key_bind(&mut vec, KEY_CUT, &LANG.cut);
            self.set_key_bind(&mut vec, KEY_GREP, &LANG.grep);
            self.set_key_bind_wide(&mut vec, KEY_RECORD_START, &LANG.key_record_start);
            self.set_key_bind_wide(&mut vec, KEY_RECORD_STOP, &LANG.key_record_stop);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 4th line
            self.set_key_bind_ex(&mut vec, KEY_SELECT, &LANG.range_select, KEY_SELECT.chars().count() + 1, (Help::KEY_WIDTH * 2 + Help::KEY_FUNC_WIDTH * 2) - (KEY_SELECT.chars().count() + 1));
            self.set_key_bind_wide(&mut vec, KEY_ALL_SELECT, &LANG.all_select);
            self.set_key_bind_wide(&mut vec, KEY_HELP, &format!("{} {}", &LANG.help, &LANG.end));
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            /* // 5th line
               self.set_key_bind_ex(&mut vec, KEY_HELP, &format!("{} {}", &LANG.help, &LANG.end), Help::KEY_FUNC_WIDTH, Help::KEY_WIDTH);
               self.set_key_bind_ex(&mut vec, HELP_DETAIL, &env!("CARGO_PKG_REPOSITORY").to_string(), HELP_DETAIL.chars().count() + 1, Help::KEY_FUNC_WIDTH_WIDE - HELP_DETAIL.chars().count() + 1);
               self.key_bind_vec.push(vec.clone());
               vec.clear();
            */
        }

        for (i, sy) in (0_usize..).zip(self.disp_row_posi - 1..self.disp_row_posi - 1 + self.disp_row_num) {
            str_vec.push(format!("{}{}", MoveTo(0, sy as u16), Clear(ClearType::CurrentLine)));
            // Blank line to leave one line interval
            if i > 0 {
                if let Some(vec) = self.key_bind_vec.get(i - 1) {
                    for bind in vec {
                        str_vec.push(format!("{}{}", bind.key, bind.func));
                    }
                }
            }
        }
    }

    pub fn set_key_bind(&mut self, vec: &mut Vec<KeyBind>, key: &'static str, func: &String) {
        self.set_key_bind_ex(vec, key, func, Help::KEY_WIDTH, Help::KEY_FUNC_WIDTH);
    }
    pub fn set_key_bind_wide(&mut self, vec: &mut Vec<KeyBind>, key: &'static str, func: &String) {
        self.set_key_bind_ex(vec, key, func, Help::KEY_WIDTH_WIDE, Help::KEY_FUNC_WIDTH_WIDE);
    }

    pub fn set_key_bind_ex(&mut self, vec: &mut Vec<KeyBind>, key: &'static str, func: &String, key_width: usize, key_func_width: usize) {
        let key = format!("{s:<w$}", s = key, w = key_width);
        let func = format!("{s:^w$}", s = func, w = key_func_width - (get_str_width(&func) - func.chars().count()));
        vec.push(KeyBind {
            key: format!("{}{}", Colors::get_msg_highlight_fg(), key),
            func: format!("{}{}", Colors::get_msg_normal_fg(), func),
        });
    }
}
