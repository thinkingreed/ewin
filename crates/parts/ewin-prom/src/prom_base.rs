use super::{
    cont::parts::{choice::*, file_list::PromContFileList, info::*, input_area::*, key_desc::*, pulldown::*, search_opt::*},
    model::*,
    prom_trait::cont_trait::*,
};
use crate::util::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, evt::*};
use ewin_key::key::cmd::{Cmd, *};

impl PromBase {
    pub fn ctrl_cont(&mut self) -> ActType {
        Log::debug_key("PromPluginBase.ctrl_cont");

        // check allow p_cmd
        if !self.check_all_allow_cmd() {
            return ActType::Draw(DParts::MsgBar(Lang::get().unsupported_operation.to_string()));
        }
        self.set_org_state();

        // MouseDownLeft Change target
        if let CmdType::MouseDownLeft(y, x) = self.cmd.cmd_type {
            self.set_curt_cont_from_y(y, x);
            if self.curt_cont_idx != self.curt_cont_idx_org {
                self.clear_sels();
            }
        };

        // search_opt
        if let Some(search_opt) = self.get_search_opt() {
            let evt_act = search_opt.proc_search_opt();
            if evt_act != ActType::Next {
                return evt_act;
            }
        };
        // updown content
        if self.cfg.is_updown_valid {
            let act_type = self.updown_cont();
            if ActType::Next != act_type {
                return act_type;
            }
        }
        // curt_cont(input_area, ..)
        let evt_act = self.curt_input_area_proc();
        if evt_act != ActType::Next {
            return evt_act;
        }
        return ActType::Next;
    }

    pub fn get_curt_cont_mut(&mut self) -> Option<&mut Box<dyn PromContTrait>> {
        for (idx, cont) in self.cont_vec.iter_mut().enumerate() {
            if idx == self.curt_cont_idx {
                return Some(cont);
            }
        }
        return None;
    }

    pub fn curt_input_area_proc(&mut self) -> ActType {
        Log::debug_key("PromPluginTrait.curt_cont_proc");

        if let Some(cont) = self.get_curt_cont_mut() {
            if let Ok(input) = cont.downcast_mut::<PromContInputArea>() {
                let evt_act = input.proc_input_area();
                Log::debug("evt_act", &evt_act);
                if !is_select_proc(&self.cmd.cmd_type) {
                    self.clear_sels();
                }
                if evt_act != ActType::Next {
                    return evt_act;
                }
            }
        }
        return ActType::Next;
    }

    pub fn get_tgt_cont(&mut self, i: usize) -> Option<&mut Box<dyn PromContTrait>> {
        if let Some(cont) = self.cont_vec.get_mut(i) {
            return Some(cont);
        }
        return None;
    }

    pub fn get_tgt<T: PromContTrait>(&mut self, i: usize) -> &mut T {
        return self.cont_vec.get_mut(i).unwrap().downcast_mut::<T>().unwrap();
    }
    pub fn set_curt_cont_from_y(&mut self, y: usize, x: usize) {
        if let Ok(p_cont) = self.get_curt_cont_mut().unwrap().downcast_mut::<PromContPulldown>() {
            if p_cont.pulldown.is_show {
                if !p_cont.pulldown.menulist.is_mouse_within_area(y, x) {
                    p_cont.pulldown.is_show = false;
                    self.set_curt_cont_from_y_detail(y);
                }
            } else {
                self.set_curt_cont_from_y_detail(y);
            }
        } else {
            self.set_curt_cont_from_y_detail(y);
        }
    }

    pub fn set_curt_cont_from_y_detail(&mut self, y: usize) {
        for (idx, cont) in self.cont_vec.iter_mut().enumerate() {
            if (cont.downcast_ref::<PromContInputArea>().is_ok() || cont.downcast_ref::<PromContPulldown>().is_ok() || cont.downcast_ref::<PromContFileList>().is_ok()) && cont.as_base().row_posi_range.start <= y && y <= cont.as_base().row_posi_range.end {
                self.curt_cont_idx = idx;
                break;
            }
        }
    }

    pub fn get_next_cont(&mut self, is_next: bool, curt_idx: usize) -> Option<&mut Box<dyn PromContTrait>> {
        if let Some(next_idx) = self.get_next_cont_idx(is_next, curt_idx) {
            return self.get_tgt_cont(next_idx);
        }
        return None;
    }
    pub fn get_next_cont_idx(&self, is_next: bool, curt_idx: usize) -> Option<usize> {
        Log::debug_key("PromptPluginBase.get_next_cont");
        Log::debug("curt_idx", &curt_idx);

        let mut menu_vec = self.cont_vec.clone();

        if !is_next {
            menu_vec.reverse();
        }
        let (first, second) = if is_next { (menu_vec[curt_idx + 1..].to_vec(), menu_vec[..curt_idx].to_vec()) } else { (menu_vec[menu_vec.len() - curt_idx..].to_vec(), menu_vec[..menu_vec.len() - curt_idx].to_vec()) };
        Log::debug("first", &first);
        Log::debug("second", &second);

        if let Some(next_idx) = self.get_next_cont_proc(first) {
            Log::debug("next_idx 111", &next_idx);
            return Some(if is_next { curt_idx + next_idx + 1 } else { curt_idx - next_idx - 1 });
        }
        if let Some(next_idx) = self.get_next_cont_proc(second) {
            Log::debug("next_idx 222", &next_idx);
            return Some(if is_next { next_idx } else { menu_vec.len() - 1 - next_idx });
        }
        return None;
    }

    fn get_next_cont_proc(&self, vec: Vec<Box<dyn PromContTrait>>) -> Option<usize> {
        Log::debug_key("get_next_cont_proc");
        for (idx, cont) in (0_usize..).zip(vec.iter()) {
            if cont.downcast_ref::<PromContInputArea>().is_ok() {
                Log::debug("PromContInputArea", &idx);
                return Some(idx);
            } else if let Ok(cont) = cont.downcast_ref::<PromContChoice>() {
                // if cont.downcast_ref::<PromContChoice>().is_ok() {
                if cont.is_disp {
                    Log::debug("PromContChoice", &idx);
                    return Some(idx);
                }
            } else if cont.downcast_ref::<PromContPulldown>().is_ok() {
                Log::debug("PromContInputArea", &idx);
                return Some(idx);
            } else if cont.downcast_ref::<PromContFileList>().is_ok() {
                Log::debug("PromContFileList", &idx);
                return Some(idx);
            }
        }
        return None;
    }
    pub fn is_draw_cur(&mut self) -> bool {
        if let Some(cont) = self.get_curt_cont_mut() {
            if cont.downcast_mut::<PromContInputArea>().is_ok() {
                return true;
            }
        }
        return false;
    }

    pub fn get_cur_posi(&mut self) -> Option<(usize, usize)> {
        if let Some(ref curt_cont) = self.get_curt_cont_mut() {
            if let Ok(input_area) = curt_cont.downcast_ref::<PromContInputArea>() {
                return Some((input_area.base.row_posi_range.start + input_area.desc_str_vec.len(), input_area.cur.disp_x));
            }
        }
        return None;
    }

    pub fn clear_sels(&mut self) {
        self.proc_for_input_area(|input_area| input_area.sel.clear());
    }

    pub fn set_key_info(&mut self, cmd: Cmd) {
        self.cmd = cmd.clone();
        for item in self.cont_vec.iter_mut() {
            item.as_mut_base().cmd = cmd.clone();
        }
    }

    pub fn get_disp_all_row_num(&mut self, row_bottom_posi: usize) -> usize {
        let mut len = 0;
        for cont in self.cont_vec.iter_mut() {
            len += if let Ok(guide) = cont.downcast_ref::<PromContInfo>() {
                guide.desc_str_vec.len()
            } else if let Ok(key_desc) = cont.downcast_ref::<PromContKeyDesc>() {
                key_desc.desc_vecs.len()
            } else if cont.downcast_ref::<PromContSearchOpt>().is_ok() {
                1
            } else if let Ok(input_area) = cont.downcast_ref::<PromContInputArea>() {
                input_area.desc_str_vec.len() + 1
            } else if let Ok(choice) = cont.downcast_ref::<PromContChoice>() {
                choice.desc_str_vec.len() + choice.vec.len()
            } else if let Ok(pulldown) = cont.downcast_ref::<PromContPulldown>() {
                pulldown.desc_str_vec.len() + 1
            } else if let Ok(file_list) = cont.downcast_mut::<PromContFileList>() {
                // 5 = MsgBar + ContInfo + ContKey + ContInputArea
                let row_num = row_bottom_posi - 5;
                file_list.row_num = row_num;
                row_num
            } else {
                0
            }
        }
        return len;
    }

    pub fn set_cont_item_disp_posi(&mut self, disp_row_posi: usize) {
        let mut posi = disp_row_posi;
        for item in self.cont_vec.iter_mut() {
            if let Ok(guide) = item.downcast_mut::<PromContInfo>() {
                guide.base.row_posi_range = posi..posi + guide.desc_str_vec.len();
                posi += guide.desc_str_vec.len();
            } else if let Ok(key_desc) = item.downcast_mut::<PromContKeyDesc>() {
                key_desc.base.row_posi_range = posi..posi + key_desc.desc_vecs.len() - 1;
                posi += key_desc.desc_vecs.len();
            } else if let Ok(search_opt) = item.downcast_mut::<PromContSearchOpt>() {
                search_opt.base.row_posi_range = posi..posi;
                posi += 1;
            } else if let Ok(input_area) = item.downcast_mut::<PromContInputArea>() {
                input_area.base.row_posi_range = posi..posi + input_area.desc_str_vec.len();
                posi += 1 + input_area.desc_str_vec.len();
            } else if let Ok(choice) = item.downcast_mut::<PromContChoice>() {
                choice.base.row_posi_range = posi..posi + choice.desc_str_vec.len() + choice.vec.len() - 1;
                posi += choice.desc_str_vec.len() + choice.vec.len();
            } else if let Ok(pulldown) = item.downcast_mut::<PromContPulldown>() {
                pulldown.base.row_posi_range = posi..posi + pulldown.desc_str_vec.len();
                posi += pulldown.desc_str_vec.len();
                Log::debug("pulldown.base.row_posi_range", &pulldown.base.row_posi_range);
            } else if let Ok(file_list) = item.downcast_mut::<PromContFileList>() {
                file_list.base.row_posi_range = posi..posi + file_list.row_num - 1;
                posi += file_list.desc_str_vec.len() + file_list.vec.len();
            }
        }
    }
    pub fn updown_cont(&mut self) -> ActType {
        Log::debug_key("updown_cont");
        Log::debug("self.curt_cont_idx", &self.curt_cont_idx);

        let mut is_next = false;
        let curt_cont = self.get_curt_cont_mut().unwrap().clone();
        Log::debug("curt_cont", &curt_cont);
        if let Ok(curt_input_area) = curt_cont.downcast_ref::<PromContInputArea>() {
            if curt_input_area.config.is_path {
                match self.cmd.cmd_type {
                    CmdType::CursorUp | CmdType::CursorDown => {}
                    _ => return ActType::Next,
                };
            } else {
                match self.cmd.cmd_type {
                    CmdType::CursorUp | CmdType::CursorDown | CmdType::NextContent | CmdType::BackContent => {}
                    _ => return ActType::Next,
                };
            }
            is_next = self.cmd.cmd_type == CmdType::CursorDown || self.cmd.cmd_type == CmdType::NextContent;
        } else if let Ok(choice) = curt_cont.downcast_ref::<PromContChoice>() {
            if choice.config.is_multi_row {
                match self.cmd.cmd_type {
                    CmdType::NextContent | CmdType::BackContent => {}
                    _ => return ActType::Next,
                };
                is_next = self.cmd.cmd_type == CmdType::NextContent
            } else {
                match self.cmd.cmd_type {
                    CmdType::CursorUp | CmdType::CursorDown | CmdType::NextContent | CmdType::BackContent => {}
                    _ => return ActType::Next,
                };
                is_next = self.cmd.cmd_type == CmdType::CursorDown || self.cmd.cmd_type == CmdType::NextContent
            }
        } else if curt_cont.downcast_ref::<PromContPulldown>().is_ok() {
            match self.cmd.cmd_type {
                CmdType::CursorUp | CmdType::CursorDown | CmdType::NextContent | CmdType::BackContent => {}
                _ => return ActType::Next,
            };
            is_next = self.cmd.cmd_type == CmdType::CursorDown || self.cmd.cmd_type == CmdType::NextContent
        } else if curt_cont.downcast_ref::<PromContFileList>().is_ok() {
            return ActType::Next;
        };
        Log::debug("is_next", &is_next);
        Log::debug("self.curt_cont_idx 111", &self.curt_cont_idx);

        // PromContFileList is individual processing
        if let Some(next_cont) = self.get_next_cont(is_next, self.curt_cont_idx) {
            if next_cont.downcast_ref::<PromContFileList>().is_ok() {
                return ActType::Next;
            }
        }

        self.curt_cont_idx = self.get_next_cont_idx(is_next, self.curt_cont_idx).unwrap();
        Log::debug("self.curt_cont_idx 222", &self.curt_cont_idx);
        let next_cont = self.get_curt_cont_mut().unwrap();
        Log::debug("next_cont", &next_cont);

        if let Ok(next_input_area) = next_cont.downcast_mut::<PromContInputArea>() {
            if let Ok(curt_input_area) = curt_cont.downcast_ref::<PromContInputArea>() {
                next_input_area.set_cur_target(curt_input_area.cur.disp_x);
            }
        }
        if next_cont.downcast_mut::<PromContFileList>().is_ok() {
            return ActType::Next;
        }

        return ActType::Draw(DParts::Prompt);
    }

    pub fn check_all_allow_cmd(&self) -> bool {
        Log::debug_key("PromPluginBase.check_all_allow_p_cmd");
        for cont in self.cont_vec.iter() {
            if cont.check_allow_p_cmd() {
                return true;
            }
        }
        return false;
    }
    pub fn set_next_back_cont_idx(&mut self) {
        self.curt_cont_idx = self.get_next_cont_idx(self.cmd.cmd_type == CmdType::CursorDown, self.curt_cont_idx).unwrap();
    }

    pub fn set_org_state(&mut self) {
        Log::debug_key("set_org_state");
        self.curt_cont_idx_org = self.curt_cont_idx
    }
}
