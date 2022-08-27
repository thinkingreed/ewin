use ewin_const::def::USIZE_UNDEFINED;

impl TabsAllState {
    pub fn set_idx_close_other_than_this_tab(&mut self, del_idx: usize) {
        if self.close_other_than_this_tab_idx != USIZE_UNDEFINED && del_idx < self.close_other_than_this_tab_idx && self.close_other_than_this_tab_idx > 0 {
            self.close_other_than_this_tab_idx -= 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsAllState {
    pub is_all_close_confirm: bool,
    pub is_all_save: bool,
    pub close_other_than_this_tab_idx: usize,
}

impl Default for TabsAllState {
    fn default() -> Self {
        TabsAllState { is_all_close_confirm: false, is_all_save: false, close_other_than_this_tab_idx: USIZE_UNDEFINED }
    }
}
