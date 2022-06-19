use super::{model::*, prom_trait::main_trait::*};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use ewin_cfg::log::*;
use ewin_com::{
    _cfg::key::cmd::{Cmd, CmdType},
    model::*,
};
use std::{io::Write, u16};

impl Prom {
    pub fn init(&mut self, plugin: Box<dyn PromPluginTrait>) {
        self.curt = plugin;
        self.set_size();
    }

    pub fn curt<T: PromPluginTrait>(&mut self) -> &mut T {
        return self.curt.downcast_mut::<T>().unwrap();
    }

    pub fn set_size(&mut self) {
        self.row_num = self.curt.as_mut_base().get_disp_all_row_num(self.row_bottom_posi);

        Log::debug("self.row_num", &self.row_num);
        Log::debug("self.row_bottom_posi", &self.row_bottom_posi);
        self.row_posi = self.row_bottom_posi - self.row_num;
        self.curt.as_mut_base().set_cont_item_disp_posi(self.row_posi);
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>, tab_state: &TabState) {
        Log::info_key("Prompt.draw");
        Log::debug("tab_state", &tab_state);
        Log::debug("self.curt.as_base().curt_cont_idx", &self.curt.as_base().curt_cont_idx);

        if !tab_state.is_nomal_and_not_result() {
            for (i, cont) in self.curt.as_base().cont_vec.iter().enumerate() {
                Log::debug("iiiii", &i);
                Log::debug("cont.as_base().row_posi_range", &cont.as_base().row_posi_range);
                for i in cont.as_base().row_posi_range.start..=cont.as_base().row_posi_range.end {
                    str_vec.push(format!("{}{}", MoveTo(0, (i) as u16), Clear(CurrentLine),));
                }
                str_vec.push(MoveTo(0, cont.as_base().row_posi_range.start as u16).to_string());

                let is_curt = i == self.curt.as_base().curt_cont_idx;
                cont.draw(str_vec, is_curt);
            }
        }
    }

    pub fn resize(&mut self) -> ActType {
        match self.cmd.cmd_type {
            CmdType::Resize(_, _) => self.set_size(),
            _ => return ActType::Next,
        }
        return ActType::Draw(DParts::All);
    }

    pub fn clear(&mut self) {
        Log::debug_key("Prompt.clear");
        self.row_num = 0;
        self.row_posi = 0;
        self.col_num = 0;
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T, tab_state: &TabState) {
        Log::debug_key("Prompt.draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v, tab_state);
        self.draw_cur(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>) -> bool {
        if let Some((y, x)) = self.curt.as_mut_base().get_cur_posi() {
            str_vec.push(MoveTo(x as u16, y as u16).to_string());
            return true;
        }
        return false;
    }

    pub fn set_cmd(&mut self, cmd: &Cmd) {
        Log::debug_key("Prompt::set_keys");

        self.cmd = cmd.clone();
        self.curt.as_mut_base().set_key_info(cmd.clone());
    }
}
