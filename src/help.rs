use crate::{bar::headerbar::*, bar::msgbar::*, bar::statusbar::StatusBar, colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, terminal::*, util::*};
use crossterm::{cursor::*, terminal::*};
use unicode_width::UnicodeWidthChar;

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
    pub func_nm: String,
    pub key_bind_len: usize,
    pub mouse_area: (usize, usize),
}

impl Help {
    pub const DISP_ROW_NUM: usize = 5;

    const KEY_WIDTH: usize = 9;
    const KEY_WIDTH_WIDE: usize = 11;
    const KEY_FUNC_WIDTH: usize = 9;
    const KEY_FUNC_WIDTH_WIDE: usize = 12;

    pub fn new() -> Self {
        return Help { ..Help::default() };
    }

    pub fn disp_toggle(&mut self, hbar: &mut HeaderBar, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) {
        self.mode = match self.mode {
            HelpMode::Show => HelpMode::None,
            HelpMode::None => HelpMode::Show,
        };

        if self.mode == HelpMode::Show {
            Terminal::set_disp_size(hbar, editor, mbar, prom, self, sbar);
            // Cursor moves out of help display area
            if editor.cur.y - editor.offset_y > editor.disp_row_num - 1 {
                editor.cur.y = editor.offset_y + editor.disp_row_num - 1;
                editor.cur.x = editor.rnw;
                editor.cur.disp_x = editor.rnw + 1;
            }
            editor.d_range.draw_type = DrawType::All;
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
            self.set_key_bind_wide(&mut vec, KEY_HELP_DETAIL, &format!("{} {}", &LANG.help, &LANG.detail));
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
            if i == 0 {
                continue;
            }
            if let Some(vec) = self.key_bind_vec.get(i - 1) {
                for bind in vec {
                    str_vec.push(format!("{}{}", bind.key, bind.func_nm));
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

        let mut key_w = 0;
        for c in key.chars() {
            key_w += c.width().unwrap_or(0);
        }
        let mut func_w = 0;
        for c in func.chars() {
            func_w += c.width().unwrap_or(0);
        }
        let mut row_w = 0;
        for key_bind in vec.iter() {
            row_w += key_bind.key_bind_len;
        }

        let key_bind = KeyBind {
            key: format!("{}{}", Colors::get_msg_highlight_fg(), key),
            func_nm: format!("{}{}", Colors::get_msg_normal_fg(), func),
            key_bind_len: key_w + func_w,
            mouse_area: (row_w, row_w + key_w - 1),
        };

        Log::ep("key_bind", &key_bind.clone());

        vec.push(key_bind);
    }
}
