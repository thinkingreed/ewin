use crate::prompt::promptcont::promptcont::*;

impl PromptCont {
    pub fn paste(&mut self, clipboard: &String) -> bool {
        if self.sel.is_selected() {
            self.del_sel_range();
            self.sel.clear();
        }
        let chars: Vec<char> = clipboard.chars().collect();
        for c in chars {
            self.buf.insert(self.cur.x, c);
            self.cur_right();
        }
        return false;
    }
}
