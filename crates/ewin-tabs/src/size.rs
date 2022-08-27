use crate::tabs::Tabs;
use ewin_cfg::log::*;
use ewin_const::{def::*, term::*};
use ewin_file_bar::filebar::*;
use ewin_key::model::*;
use ewin_state::term::*;

impl Tabs {
    pub fn set_size(&mut self) -> bool {
        Log::debug_s("set_size");
        let (cols, rows) = get_term_size();
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));

        FileBar::set_posi(cols);
        FileBar::set_filenm();

        //  let help = Help::get();
        //  let help_row_num = if help.is_show { help.row_num } else { 0 };
        let help_row_num = 0;

        self.curt().sbar.row_posi = if help_row_num == 0 { rows - 1 } else { rows - help_row_num - 1 };
        self.curt().sbar.col_num = cols;

        let prom_row_num = if State::get().curt_state().is_nomal() { 0 } else { self.curt().prom.row_num };
        self.curt().prom.col_num = cols;

        self.curt().prom.row_posi = rows - prom_row_num - help_row_num - self.curt().sbar.row_num;
        self.curt().msgbar.col_num = cols;
        self.curt().msgbar.row_num = MSGBAR_ROW_NUM;

        self.curt().msgbar.row_posi = rows - prom_row_num - help_row_num - self.curt().sbar.row_num - 1;

        if State::get().curt_state().prom == PromState::OpenFile {
            self.curt().editor.view.height = 0;
        } else {
            let scale_row_num = if State::get().curt_state().editor.scale.is_enable { SCALE_ROW_NUM } else { 0 };
            Log::debug("scale_row_num", &scale_row_num);
            Log::debug(" self.curt().msgbar.row_num", &self.curt().msgbar.row_num);
            Log::debug(" prom_row_num", &prom_row_num);
            Log::debug(" self.curt().sbar.row_num", &self.curt().sbar.row_num);
            let editor_row = rows - if State::get().curt_state().prom == PromState::OpenFile { 0 } else { MSGBAR_ROW_NUM } - FileBar::get().row_num - prom_row_num - self.curt().msgbar.row_num - help_row_num - self.curt().sbar.row_num;
            self.curt().editor.set_size_editor(editor_row, scale_row_num);
        }

        return true;
    }
}
