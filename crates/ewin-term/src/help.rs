use crate::{global_term::*, model::*};
use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_com::{
    _cfg::key::{cmd::*, keybind::*},
    model::*,
    util::*,
};
use ewin_const::def::*;

impl Help {
    pub const DISP_ROW_NUM: usize = 5;
    const KEY_WIDTH: usize = 7;
    const KEY_WIDTH_WIDE: usize = 9;
    const KEY_FUNC_WIDTH: usize = 9;
    const KEY_FUNC_WIDTH_WIDE: usize = 14;

    pub fn disp_toggle(term: &mut Terminal) {
        toggle_help_disp();
        // term.help.is_disp = !term.help.is_disp;
        term.set_disp_size();

        let tab = term.tabs.get_mut(term.tab_idx).unwrap();
        if HELP_DISP.get().unwrap().try_lock().unwrap().is_disp {
            // Cursor moves out of help display area
            if tab.editor.cur.y - tab.editor.offset_y > tab.editor.row_len - 1 {
                tab.editor.cur.y = tab.editor.offset_y + tab.editor.row_len - 1;
                tab.editor.cur.x = 0;
                tab.editor.cur.disp_x = 0;
            }
        }
        tab.editor.draw_range = E_DrawRange::All;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::info_key("Help.draw");

        if !self.is_disp {
            return;
        }
        if self.key_bind_vec.is_empty() {
            let mut vec: Vec<HelpKeybind> = vec![];

            // 1st line
            self.set_key_bind(&mut vec, &Keybind::get_key_str(CmdType::CloseFile), &Lang::get().close);
            self.set_key_bind(&mut vec, &Keybind::get_key_str(CmdType::SaveFile), &Lang::get().save);
            self.set_key_bind_wide(&mut vec, &Keybind::get_key_str(CmdType::Copy), &Lang::get().copy);
            self.set_key_bind_wide(&mut vec, &Keybind::get_key_str(CmdType::InsertStr("".to_string())), &Lang::get().paste);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 2nd line
            self.set_key_bind(&mut vec, &Keybind::get_key_str(CmdType::Undo), &Lang::get().undo);
            self.set_key_bind(&mut vec, &Keybind::get_key_str(CmdType::Redo), &Lang::get().redo);
            self.set_key_bind_wide(&mut vec, &Keybind::get_key_str(CmdType::FindNext), &Lang::get().search);
            self.set_key_bind_wide(&mut vec, &Keybind::get_key_str(CmdType::ReplaceProm), &Lang::get().replace);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 3rd line
            self.set_key_bind(&mut vec, &Keybind::get_key_str(CmdType::Cut), &Lang::get().cut);
            self.set_key_bind(&mut vec, &Keybind::get_key_str(CmdType::GrepProm), &Lang::get().grep);
            self.set_key_bind_wide(&mut vec, &Keybind::get_key_str(CmdType::RecordKeyStartEnd), &Lang::get().key_record_start_stop);
            self.set_key_bind_wide(&mut vec, &Keybind::get_key_str(CmdType::ExecRecordKey), &Lang::get().key_record_exec);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 4th line
            let key_select_str = &format!("{}{}", Keybind::get_key_str(CmdType::CursorLeftSelect).split('+').collect::<Vec<&str>>()[0], KEY_SELECT_KEY);
            self.set_key_bind_ex(&mut vec, key_select_str, &Lang::get().range_select, key_select_str.chars().count() + 1, (Help::KEY_WIDTH * 2 + Help::KEY_FUNC_WIDTH * 2) - (key_select_str.chars().count() + 1));
            self.set_key_bind_wide(&mut vec, &Keybind::get_key_str(CmdType::MouseModeSwitch), &Lang::get().mouse_switch);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
            // 5th line
            self.set_key_bind_ex(&mut vec, &Keybind::get_key_str(CmdType::Help), &format!("{}{}", &Lang::get().help, &Lang::get().end), Help::KEY_FUNC_WIDTH, Help::KEY_WIDTH);
            self.set_key_bind_ex(&mut vec, &Keybind::get_key_str(CmdType::HelpInitDisplaySwitch), &Lang::get().help_init_display_switch, Help::KEY_FUNC_WIDTH, Help::KEY_WIDTH);
            self.key_bind_vec.push(vec.clone());
            vec.clear();
        }

        for (i, sy) in (0_usize..).zip(self.row_posi..self.row_posi + self.row_num) {
            str_vec.push(format!("{}{}", MoveTo(0, sy as u16), Clear(ClearType::CurrentLine)));

            if let Some(vec) = self.key_bind_vec.get(i) {
                let mut row_str = String::new();
                let mut width = 0;
                for bind in vec {
                    if width + get_str_width(&bind.key) <= self.col_num {
                        row_str.push_str(&format!("{}{}", Colors::get_msg_highlight_fg(), bind.key));
                        width += get_str_width(&bind.key);

                        if width + get_str_width(&bind.funcnm) <= self.col_num {
                            row_str.push_str(&format!("{}{}", Colors::get_msg_normal_fg(), bind.funcnm));
                            width += get_str_width(&bind.funcnm);
                        } else {
                            let funcnm = cut_str(&bind.funcnm, self.col_num - width, false, false);
                            row_str.push_str(&format!("{}{}", Colors::get_msg_normal_fg(), funcnm));
                            break;
                        }
                    } else {
                        let key = cut_str(&bind.key, self.col_num - width, false, false);
                        row_str.push_str(&format!("{}{}", Colors::get_msg_highlight_fg(), key));
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
    pub fn new() -> Self {
        Help { ..Help::default() }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Help {
    pub is_disp: bool,
    // Number displayed on the terminal
    pub row_num: usize,
    pub col_num: usize,
    // 0 index
    pub row_posi: usize,
    pub key_bind_vec: Vec<Vec<HelpKeybind>>,
}
/*
impl Default for Help {
    fn default() -> Self {
        Help { is_disp: false, col_num: 0, row_num: 0, row_posi: 0, key_bind_vec: vec![] }
    }
}
 */
#[derive(Debug, Clone)]
pub struct HelpKeybind {
    pub key: String,
    pub funcnm: String,
    pub key_bind_len: usize,
    pub mouse_area: (usize, usize),
}
