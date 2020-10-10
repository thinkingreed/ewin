use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Log, MsgBar};
use termion::{clear, cursor};

impl MsgBar {
    pub fn new(lang_cfg: LangCfg) -> Self {
        MsgBar { lang: lang_cfg, ..MsgBar::default() }
    }

    pub fn clear(&mut self) {
        Log::ep_s("★　MsgBar.clear");
        self.msg_disp = String::new();
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("★　MsgBar.draw");
        if self.msg_disp.len() > 0 {
            let msg = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi) as u16), clear::CurrentLine, self.msg_disp.clone());
            str_vec.push(msg);
        }
    }
}
