use crate::{def::*, global::*, log::*, model::*, msgbar::*, prompt::promptcont::promptcont::*};

impl PromptCont {
    pub fn paste(&mut self, editor: &mut Editor, mbar: &mut MsgBar) -> bool {
        Log::ep_s("　　　　　　　PromptCont.paste");
        if self.sel.is_selected() {
            self.del_sel_range();
            self.sel.clear();
        }
        let contexts = editor.get_clipboard().unwrap_or("".to_string());
        Log::ep("contexts", &contexts);
        if contexts.match_indices(NEW_LINE).count() == 0 {
            let chars: Vec<char> = contexts.chars().collect();
            for c in chars {
                self.buf.insert(self.cur.x, c);
                self.cur_right();
            }
            return false;
        } else {
            mbar.set_err(&LANG.cannot_paste_multi_rows.clone());
            return true;
        }
    }
}
