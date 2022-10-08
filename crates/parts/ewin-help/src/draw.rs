use crate::help::*;
use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_const::{def::*, models::file::*};
use ewin_key::key::{cmd::*, keys::*};
use ewin_utils::{char_edit::*, str_edit::*};
use std::{fmt::Write as _, io::Write};

impl Help {
    pub const DISP_ROW_NUM: usize = 5;
    const KEY_WIDTH: usize = 7;
    const KEY_WIDTH_WIDE: usize = 9;
    const KEY_FUNC_WIDTH: usize = 9;
    const KEY_FUNC_WIDTH_WIDE: usize = 14;

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::info_key("Help.draw");

        if !self.is_show {
            return;
        }
        if self.key_bind_vec.is_empty() {
            let mut vec: Vec<HelpKeybind> = vec![];

            // 1st line
            self.set_key_bind(&mut vec, &Keys::get_key_str(CmdType::CloseFileCurt(CloseFileType::Normal)), &Lang::get().close);
            self.set_key_bind(&mut vec, &Keys::get_key_str(CmdType::SaveFile(SaveFileType::Normal)), &Lang::get().save);
            self.set_key_bind_wide(&mut vec, &Keys::get_key_str(CmdType::Copy), &Lang::get().copy);
            self.set_key_bind_wide(&mut vec, &Keys::get_key_str(CmdType::InsertStr("".to_string())), &Lang::get().paste);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 2nd line
            self.set_key_bind(&mut vec, &Keys::get_key_str(CmdType::Undo), &Lang::get().undo);
            self.set_key_bind(&mut vec, &Keys::get_key_str(CmdType::Redo), &Lang::get().redo);
            self.set_key_bind_wide(&mut vec, &Keys::get_key_str(CmdType::FindNext), &Lang::get().search);
            self.set_key_bind_wide(&mut vec, &Keys::get_key_str(CmdType::ReplaceProm), &Lang::get().replace);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 3rd line
            self.set_key_bind(&mut vec, &Keys::get_key_str(CmdType::Cut), &Lang::get().cut);
            self.set_key_bind(&mut vec, &Keys::get_key_str(CmdType::GrepProm), &Lang::get().grep);
            self.set_key_bind_wide(&mut vec, &Keys::get_key_str(CmdType::RecordKeyStartEnd), &Lang::get().key_record_start_stop);
            self.set_key_bind_wide(&mut vec, &Keys::get_key_str(CmdType::ExecRecordKey), &Lang::get().key_record_exec);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 4th line
            let key_select_str = &format!("{}{}", Keys::get_key_str(CmdType::CursorLeftSelect).split('+').collect::<Vec<&str>>()[0], KEY_SELECT_KEY);
            self.set_key_bind_ex(&mut vec, key_select_str, &Lang::get().range_select, key_select_str.chars().count() + 1, (Help::KEY_WIDTH * 2 + Help::KEY_FUNC_WIDTH * 2) - (key_select_str.chars().count() + 1));
            self.set_key_bind_wide(&mut vec, &Keys::get_key_str(CmdType::MouseModeSwitch), &Lang::get().mouse_switch);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 5th line
            self.set_key_bind_ex(&mut vec, &Keys::get_key_str(CmdType::Help), &format!("{}{}", &Lang::get().help, &Lang::get().end), Help::KEY_FUNC_WIDTH, Help::KEY_WIDTH);
            self.set_key_bind_ex(&mut vec, &Keys::get_key_str(CmdType::HelpInitDisplaySwitch), &Lang::get().help_init_display_switch, Help::KEY_FUNC_WIDTH, Help::KEY_WIDTH);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
        }

        for (i, sy) in (0_usize..).zip(self.view.y..self.view.y + self.view.height) {
            str_vec.push(format!("{}{}", MoveTo(0, sy as u16), Clear(ClearType::CurrentLine)));

            if let Some(vec) = self.key_bind_vec.get(i) {
                let mut row_str = String::new();
                let mut width = 0;
                for bind in vec {
                    if width + get_str_width(&bind.key) <= self.view.width {
                        let _ = write!(row_str, "{}{}", Colors::get_msg_highlight_fg(), bind.key);
                        width += get_str_width(&bind.key);

                        if width + get_str_width(&bind.funcnm) <= self.view.width {
                            let _ = write!(row_str, "{}{}", Colors::get_msg_normal_fg(), bind.funcnm);
                            width += get_str_width(&bind.funcnm);
                        } else {
                            let funcnm = cut_str(&bind.funcnm, self.view.width - width, false, false);
                            let _ = write!(row_str, "{}{}", Colors::get_msg_normal_fg(), funcnm);
                            break;
                        }
                    } else {
                        let key = cut_str(&bind.key, self.view.width - width, false, false);
                        let _ = write!(row_str, "{}{}", Colors::get_msg_highlight_fg(), key);
                        break;
                    }
                }

                str_vec.push(row_str.clone());
            }
        }
    }
    pub fn set_key_bind(&mut self, vec: &mut Vec<HelpKeybind>, key: &str, func: &str) {
        self.set_key_bind_ex(vec, key, func, Help::KEY_WIDTH, Help::KEY_FUNC_WIDTH);
    }

    pub fn set_key_bind_wide(&mut self, vec: &mut Vec<HelpKeybind>, key: &str, func: &str) {
        self.set_key_bind_ex(vec, key, func, Help::KEY_WIDTH_WIDE, Help::KEY_FUNC_WIDTH_WIDE);
    }

    pub fn set_key_bind_ex(&mut self, vec: &mut Vec<HelpKeybind>, key: &str, func: &str, key_width: usize, key_func_width: usize) {
        let key = format!("{s:<w$}", s = key, w = key_width);
        let func = format!("{s:^w$}", s = func, w = key_func_width - (get_str_width(func) - func.chars().count()));

        let mut key_w = 0;
        for c in key.chars() {
            key_w += get_c_width(&c);
        }
        let mut func_w = 0;
        for c in func.chars() {
            func_w += get_c_width(&c);
        }
        let mut row_w = 0;
        for key_bind in vec.iter() {
            row_w += key_bind.key_bind_len;
        }

        let key_bind = HelpKeybind { key, funcnm: func, key_bind_len: key_w + func_w, mouse_area: (row_w, row_w + key_w - 1) };

        // Log::ep("key_bind", &key_bind.clone());

        vec.push(key_bind);
    }
    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("Help::draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
