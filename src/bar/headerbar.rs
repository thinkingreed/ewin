use crate::{colors::*, def::*, global::*, log::*};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;
#[derive(Debug, Clone)]
pub struct HeaderBar {
    pub close_btn: String,
    pub close_btn_disp: String,
    pub help_btn: String,
    pub help_btn_disp: String,
    // Position on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
}

impl Default for HeaderBar {
    fn default() -> Self {
        HeaderBar {
            close_btn: "✕".to_string(),
            close_btn_disp: String::new(),
            help_btn: String::new(),
            help_btn_disp: String::new(),
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
        }
    }
}

impl HeaderBar {
    pub fn new(disp_col_num: usize) -> Self {
        HeaderBar { disp_col_num, ..HeaderBar::default() }
    }

    pub fn draw<T: Write>(&mut self, out: &mut T) {
        Log::ep_s("　　　　　　　　HeaderBar.draw");

        let (other_w, help_w, close_w) = self.get_areas_width(self.disp_col_num);

        let other = format!("{}", " ");
        let other_disp = format!("{o:^width$}", o = other, width = other_w);

        let help_btn = format!("{}:{}", KEY_HELP, LANG.help);
        let help_btn_disp = format!("{h:^width$}", h = help_btn, width = help_w);

        let close_btn = format!(" {} ", self.close_btn);
        let close_btn_disp = format!("{c:^width$}", c = close_btn, width = close_w);

        let cfg = CFG.get().unwrap().try_lock().unwrap();
        let hber_str = format!(
            "{}{}{}{}{}{}{}{} {}{}{}{}",
            MoveTo(0, self.disp_row_posi as u16),
            Clear(ClearType::CurrentLine),
            Colors::bg(cfg.colors.editor.bg),
            other_disp,
            Colors::bg(cfg.colors.status_bar.fg),
            Colors::fg(cfg.colors.editor.bg),
            help_btn_disp,
            Colors::bg(cfg.colors.editor.bg),
            Colors::bg(cfg.colors.status_bar.fg),
            Colors::fg(cfg.colors.editor.bg),
            close_btn_disp,
            Colors::bg(cfg.colors.editor.bg),
        );

        let _ = out.write(&hber_str.as_bytes());
        out.flush().unwrap();
    }

    fn get_areas_width(&self, cols_w: usize) -> (usize, usize, usize) {
        let close_btn_w_max = 3;
        let help_btn_w_max = 7;
        return (cols_w - help_btn_w_max - close_btn_w_max - 1, help_btn_w_max, close_btn_w_max);
    }
}
