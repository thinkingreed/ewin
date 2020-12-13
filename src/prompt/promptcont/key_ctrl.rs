use crate::global::*;
use crate::model::*;

impl PromptCont {
    pub fn paste(&mut self, term: &Terminal, editor: &mut Editor, mbar: &mut MsgBar) -> bool {
        Log::ep_s("　　　　　　　PromptCont.paste");
        let contexts = editor.get_clipboard(&term).unwrap_or("".to_string());
        if contexts.match_indices("\n").count() == 0 {
            let chars: Vec<char> = contexts.chars().collect();
            for c in chars {
                self.buf.insert(self.cur.x, c);
                self.cursor_right();
            }
            return false;
        } else {
            mbar.set_err(&LANG.lock().unwrap().cannot_paste_multi_lines.clone());
            return true;
        }
    }
}