use crate::{log::*, prompt::promptcont::promptcont::*};

impl PromptCont {
    pub fn paste(&mut self, clipboard: &String) -> bool {
        Log::ep_s("　　　　　　　PromptCont.paste");
        if self.sel.is_selected() {
            self.del_sel_range();
            self.sel.clear();
        }
        Log::ep("contexts", &clipboard);
        let chars: Vec<char> = clipboard.chars().collect();
        for c in chars {
            self.buf.insert(self.cur.x, c);
            self.cur_right();
        }
        return false;
    }
}
