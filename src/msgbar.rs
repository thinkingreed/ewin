use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Log, MsgBar};
use termion::{clear, color, cursor};

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
    pub fn set_msg_set_file_name(&mut self) {
        let msg = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.msg_set_file_name.clone(),);
        // let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num - (get_str_width(&msg) - msg.chars().count()));
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num);
        self.msg_disp = format!("{}{}{}", &color::Bg(color::Red).to_string(), msg_str, &color::Bg(color::Black).to_string(),);
    }
}
