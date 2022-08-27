use crate::global::*;
use crossterm::event::{Event, KeyCode::Null};
use ewin_cfg::{global::*, log::Log};
use ewin_const::models::{evt::*, term::*};
use ewin_key::key::{cmd::*, keybind::*};
use std::sync::mpsc::Sender;

impl Job {
    pub fn send_grep(tx_job: &Sender<Job>, grep_str: String, is_end: bool) {
        let _ = tx_job.send(Job { cont: JobCont::Grep(JobGrep { grep_str, is_end }) });
    }
    pub fn send_cmd(cmd_type: CmdType) {
        let cmd = Cmd::to_cmd(cmd_type.clone());
        Job::send_cmd_act_type(cmd_type, *cmd.place_vec.first().unwrap(), None);
    }
    pub fn send_cmd_str(cmd_str: &str) {
        let cmd = Cmd::to_cmd(Cmd::str_to_cmd_type(cmd_str));
        Job::send_cmd_act_type(cmd.cmd_type, *cmd.place_vec.first().unwrap(), None);
    }

    pub fn send_cmd_act_type(cmd_type: CmdType, place: Place, act_type_opt: Option<ActType>) {
        Log::debug_key("Job::send_cmd_act_type");
        if let Some(Ok(tx)) = TX_JOB.get().map(|tx| tx.try_lock()) {
            let job = Job { cont: JobCont::Cmd(JobCmd { cmd_type, place, act_type_opt }) };
            Log::debug("sendJob", &job);
            let _ = tx.send(job);
        }
    }
}

pub fn get_edit_func_str(funcnm: &str) -> Option<String> {
    Log::debug_key("get_edit_func_str");
    Log::debug("funcnm", &funcnm);
    // if let Some(name) = LANG_MAP.get(&funcnm.to_case(Case::Snake)) {
    if let Some(name) = LANG_MAP.get(funcnm) {
        Log::debug("name", &name);
        let cmd_type = Cmd::str_to_cmd_type(funcnm);
        Log::debug("cmd_type", &cmd_type);
        if cmd_type == CmdType::Unsupported {
            Some(name.clone())
        } else {
            Some(Keybind::get_menu_str(name, cmd_type))
        }
    } else {
        None
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Job {
    pub cont: JobCont,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobCont {
    Key(JobKey),
    Grep(JobGrep),
    Watch(JobWatch),
    Cmd(JobCmd),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct JobKey {
    pub evt: Event,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct JobWatch {
    pub fullpath_str: String,
    pub unixtime_str: String,
}

impl Default for JobKey {
    fn default() -> Self {
        JobKey { evt: Event::Key(Null.into()) }
    }
}

#[derive(Debug, Hash, Default, Eq, PartialEq, Clone)]
pub struct JobGrep {
    pub grep_str: String,
    pub is_end: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct JobCmd {
    pub cmd_type: CmdType,
    pub place: Place,
    pub act_type_opt: Option<ActType>,
}
