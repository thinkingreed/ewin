use crate::{def::*, global::*, model::*};

impl PromptCont {
    pub fn paste(&mut self, editor: &mut Editor, mbar: &mut MsgBar) -> bool {
        Log::ep_s("　　　　　　　PromptCont.paste");
        let contexts = editor.get_clipboard().unwrap_or("".to_string());
        Log::ep("contexts", contexts.clone());
        if contexts.match_indices(NEW_LINE).count() == 0 {
            let chars: Vec<char> = contexts.chars().collect();
            for c in chars {
                self.buf.insert(self.cur.x, c);
                self.cur_right();
            }
            return false;
        } else {
            mbar.set_err(&LANG.cannot_paste_multi_lines.clone());
            return true;
        }
    }
}
