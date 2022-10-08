use crate::{ewin_key::key::cmd::*, model::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, event::*, file::*};
use ewin_file_bar::filebar::*;
use ewin_job::job::*;
use ewin_prom::model::*;
use ewin_state::term::*;
use ewin_utils::files::file::*;
use ewin_view::view::*;
use std::path::Path;

impl Editor {
    pub fn save(&mut self, save_type: &SaveFileType) -> ActType {
        Log::debug_key("save");
        Log::debug("save_type", &save_type);

        let fullpath = State::get().curt_ref_state().file.fullpath.clone();
        let path = Path::new(&fullpath);
        let curt_idx = State::get().tabs.idx;

        match save_type {
            SaveFileType::Normal | SaveFileType::Forced | SaveFileType::Confirm if !path.exists() => {
                return Job::send_cmd(CmdType::SaveNewFileProm);
            }
            _ => {}
        };

        match save_type {
            SaveFileType::Normal | SaveFileType::Confirm => {
                // Check if the file has been updated after opening
                let fullpath = State::get().curt_ref_state().file.fullpath.clone();
                if let Some(latest_modified_time) = File::get_modified_time(&fullpath) {
                    if latest_modified_time > State::get().curt_ref_state().file.mod_time {
                        Log::debug("latest_modified_time > h_file.modified_time ", &(latest_modified_time > State::get().tabs.vec.get_mut(curt_idx).unwrap().file.mod_time));
                        return Job::send_cmd(CmdType::SaveForceProm);
                    }
                }
            }
            SaveFileType::NewFile | SaveFileType::Forced => {}
        }

        let file = State::get().curt_ref_state().file.clone();
        Log::info_s(&format!("Save {}, file info {:?}", &file.name, &file));
        let result = self.buf.write_to(&mut State::get().tabs.vec.get_mut(curt_idx).unwrap().file);
        match result {
            Ok(enc_errors) => {
                if enc_errors {
                    Log::info("Encoding errors", &enc_errors);
                    return ActType::Draw(DrawParts::TabsAllMsgBar(Lang::get().cannot_convert_encoding.to_string()));
                } else {
                    if save_type == &SaveFileType::NewFile {
                        View::set_title(&file.name);
                    }
                    let mod_time = File::get_modified_time(&State::get().tabs.vec.get_mut(curt_idx).unwrap().file.fullpath).unwrap();
                    State::get().curt_mut_state().file.mod_time = mod_time;

                    // TODO
                    // TODO
                    // TODO
                    // TODO
                    // prom state => State
                    Prom::get().clear();
                    //  self.state.clear();
                    State::get().curt_mut_state().clear();

                    let mut file_bar = FileBar::get().clone();
                    Log::info("Saved file", &file_bar.file_vec.get_mut(curt_idx).unwrap());

                    match save_type {
                        SaveFileType::Confirm => {
                            State::get().curt_mut_state().editor.is_changed = false;
                            return Job::send_cmd(CmdType::CloseFileCurt(CloseFileType::Forced));
                        }
                        SaveFileType::Normal if !State::get().curt_ref_state().editor.is_changed => return ActType::None,
                        _ => {
                            if State::get().curt_ref_state().editor.is_changed {
                                State::get().curt_mut_state().editor.is_changed = false;
                            }
                            return ActType::Draw(DrawParts::TabsAll);
                        }
                    };
                }
            }
            Err(err) => {
                Log::error("err", &err.to_string());
                return ActType::Draw(DrawParts::TabsAllMsgBar(format!("{} {:?}", Lang::get().file_saving_problem, err.kind())));
            }
        }
    }
}
