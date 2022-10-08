use std::{
    cmp::min,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{
    cont::parts::info::*,
    cont::parts::{input_area::*, key_desc::*, pulldown::*},
    ewin_key::key::cmd::*,
    model::*,
    traits::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*, model::general::default::*};
use ewin_const::models::{draw::*, event::*, file::*};
use ewin_job::job::*;
use ewin_key::model::PromState;
use ewin_state::term::*;
use ewin_utils::global::*;
use ewin_view::parts::pulldown::Pulldown;
use indexmap::*;

impl PromSaveNewFile {
    pub fn save_new_filenm(&mut self) -> ActType {
        Log::debug_key("EvtAct.save_new_filenm");

        match self.base.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                if let Ok(cont) = self.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
                    if cont.base.row_posi_range.end == y {
                        cont.pulldown.is_show = !cont.pulldown.is_show;
                    } else if cont.pulldown.menulist.is_mouse_within_area(y, x) {
                        if let Some((_, _)) = cont.pulldown.menulist.get_curt_parent() {
                            cont.pulldown.sel_idx = cont.pulldown.menulist.parent_sel_y;
                            cont.pulldown.set_sel_name();
                            if cont.pulldown.is_show {
                                cont.pulldown.is_show = false;
                            }
                        }
                    } else {
                        cont.pulldown.is_show = false;
                    }
                }
                return ActType::Draw(DrawParts::TabsAll);
            }
            CmdType::MouseMove(y, x) => {
                if let Ok(cont) = self.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
                    if cont.pulldown.menulist.is_mouse_within_area(y, x) {
                        cont.pulldown.menulist.ctrl_mouse_move(y, x);
                    }
                }
                return ActType::Draw(DrawParts::TabsAll);
            }
            CmdType::Confirm => {
                let filenm_input = self.as_mut_base().get_tgt_input_area_str(0);
                if filenm_input.is_empty() {
                    return ActType::Draw(DrawParts::MsgBar(Lang::get().not_entered_filenm.to_string()));
                } else {
                    let filenm = if Path::new(&filenm_input).extension().is_some() {
                        filenm_input
                    } else {
                        format! {"{}{}", filenm_input, self.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>().unwrap().pulldown.get_sel_name() }
                    };

                    let filenm_path = Path::new(&filenm);

                    let absolute_path = if Path::new(&filenm).is_absolute() { PathBuf::from(&filenm) } else { Path::new(&*CURT_DIR).join(&filenm) };
                    if Path::new(&absolute_path).exists() {
                        return ActType::Draw(DrawParts::MsgBar(Lang::get().file_already_exists.to_string()));
                    }

                    if filenm_path.is_absolute() {
                        State::get().curt_mut_state().file.name = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string();
                        State::get().curt_mut_state().file.fullpath = filenm.clone();
                    } else {
                        State::get().curt_mut_state().file.name = filenm.clone();
                        State::get().curt_mut_state().file.fullpath = absolute_path.to_string_lossy().to_string();
                    }
                    let ext = filenm_path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
                    State::get().curt_mut_state().file.ext = ext;

                    return Job::send_cmd(CmdType::SaveFile(SaveFileType::NewFile));
                }
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn new(candidate_new_filenm: String) -> Self {
        let mut prom = PromSaveNewFile { base: PromBase { cfg: PromptConfig { is_updown_valid: true }, ..PromBase::default() } };
        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![format!("{}", &Lang::get().set_new_filenm,)], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let confirm = PromContKeyMenu { disp_str: Lang::get().search_top.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
        let switch_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![CmdType::NextContent, CmdType::CursorUp, CmdType::CursorDown], &mut vec![CmdType::BackContent]) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![confirm, switch_area, cancel]], ..PromContKeyDesc::default() }));

        let input_area = PromContInputArea { desc_str_vec: vec![Lang::get().filenm.clone()], buf: candidate_new_filenm.chars().collect::<Vec<char>>(), ..PromContInputArea::default() };
        prom.base.cont_vec.push(Box::new(input_area));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        let mut pulldown_cont = PromContPulldown { desc_str_vec: vec![Lang::get().extension.clone()], pulldown: Pulldown::new(), ..PromContPulldown::default() };

        let mut vec = Cfg::get().general.editor.save.candidate_extension_when_saving_new_file.clone();
        let mut edit_vec = vec![];
        for s in vec.iter_mut() {
            if s.is_empty() {
                edit_vec.push(Lang::get().none.to_string());
            } else {
                edit_vec.push(format!(".{}", &s));
            };
        }
        pulldown_cont.pulldown.set_disp_name(IndexSet::from_iter(edit_vec.iter().cloned()));

        prom.base.cont_vec.push(Box::new(pulldown_cont));

        return prom;
    }

    pub fn init(candidate_new_filenm: String, editor_row_num: usize) -> ActType {
        Log::debug_key("Tab::prom_save_new_file");
        State::get().curt_mut_state().prom = PromState::SaveNewFile;
        Prom::get().init(Box::new(PromSaveNewFile::new(candidate_new_filenm)));
        if let Ok(pulldown_cont) = Prom::get().curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
            let height = min(pulldown_cont.pulldown.menulist.cont.cont_vec.len(), editor_row_num);
            pulldown_cont.pulldown.menulist.init_menu(pulldown_cont.base.row_posi_range.end, Pulldown::MARGIN, height);
        }
        return ActType::Draw(DrawParts::TabsAll);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromSaveNewFile {
    pub base: PromBase,
}
impl PromTrait for PromSaveNewFile {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
