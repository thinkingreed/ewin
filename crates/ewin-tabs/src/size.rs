use crate::tabs::*;
use ewin_cfg::log::*;
use ewin_const::{def::*, term::*};
use ewin_file_bar::filebar::*;
use ewin_key::model::*;
use ewin_msg_bar::msgbar::*;
use ewin_prom::model::*;
use ewin_side_bar::sidebar::*;
use ewin_state::term::*;

impl Tabs {
    pub fn set_size(&mut self) -> bool {
        Log::debug_s("set_size");
        let (cols, rows) = get_term_size();

        let side_bar_width = SideBar::get().get_width_all();
        Log::debug("side_bar_width", &side_bar_width);

        let cols = cols - side_bar_width;
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));

        FileBar::set_posi();
        FileBar::set_filenm();

        //  let help = Help::get();
        //  let help_row_num = if help.is_show { help.row_num } else { 0 };
        let help_row_num = 0;

        let prom_row_num = if State::get().curt_state().is_nomal() { 0 } else { Prom::get().view.height };
        Prom::get().view.width = cols;

        Prom::get().view.y = rows - prom_row_num - help_row_num - STATUSBAR_ROW_NUM;

        if State::get().curt_state().prom == PromState::OpenFile {
            self.curt().editor.view.height = 0;
        } else {
            let scale_row_num = if State::get().curt_state().editor.scale.is_enable { SCALE_HEIGHT } else { 0 };
            Log::debug("scale_row_num", &scale_row_num);
            Log::debug(" self.curt().msgbar.row_num", &MsgBar::get().view.height);
            Log::debug(" prom_row_num", &prom_row_num);
            let editor_row = rows - if State::get().curt_state().prom == PromState::OpenFile { 0 } else { MSGBAR_ROW_NUM } - FileBar::get().view.height - prom_row_num - MSGBAR_ROW_NUM - help_row_num - STATUSBAR_ROW_NUM;
            self.curt().editor.set_size_editor(editor_row, scale_row_num);
        }

        return true;
    }
}
