use crate::model::*;
use std::cmp::min;

impl PromptCont {
    pub fn new() -> Self {
        PromptCont { ..PromptCont::default() }
    }

    /// 選択箇所のhighlight
    pub fn ctl_select_color(&mut self) -> String {
        // Log::ep_s("                          Prompt.ctl_select_color");
        let ranges = self.sel.get_range();

        let mut str_vec: Vec<String> = vec![];
        for (i, c) in self.buf.iter().enumerate() {
            if ranges.sx <= i && i < ranges.ex {
                Colors::set_select_color(&mut str_vec);
                str_vec.push(c.to_string())
            } else {
                Colors::set_textarea_color(&mut str_vec);
                str_vec.push(c.to_string())
            }
        }
        Colors::set_textarea_color(&mut str_vec);

        return str_vec.join("");
    }

    pub fn del_sel_range(&mut self) {
        Log::ep_s("　　　　　　　  del_sel_range");
        let sel = self.sel.get_range();
        Log::ep("sel", &sel);
        self.buf.drain(sel.sx..sel.ex);
        self.cur.disp_x = min(sel.s_disp_x, sel.e_disp_x);
        self.cur.x = min(sel.sx, sel.ex);
    }
}
