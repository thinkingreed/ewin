use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::Colors, log::*, model::general::default::*};
use ewin_const::models::{env::*, view::get_space};
use ewin_utils::files::dir::*;
use ewin_view::view::*;
use serde::Deserialize;
use std::collections::HashMap;

use crate::sidebar::SideBarContBase;

use super::explorer::Explorer;

impl ExplorerQuickAccess {
    pub const SPLIT_LINE_WIDTH: usize = 1;

    const HOME_DIR: &str = "home";
    const CONFIG_DIR: &str = "config";
    const DESKTOP_DIR: &str = "desktop";

    pub fn get_quick_access(&mut self) -> ExplorerQuickAccess {
        Log::debug_key("ExplorerQuickAccess.get_quick_access");

        let mut quick_access = ExplorerQuickAccess::default();

        let mut map: HashMap<String, Vec<HashMap<String, ExplorerQuickAccessCont>>> = serde_json::from_str(&CfgEdit::get().general.sidebar.explorer.quick_access.content.to_string()).unwrap();

        let mut rtn_vec = vec![];
        let mut vec = map.get_mut(&get_os_str()).unwrap();

        for (idx, map) in vec.iter_mut().enumerate() {
            for (key, cont) in map.iter_mut() {
                cont.path = if let Some(ref path) = cont.path_opt { path.clone() } else { ExplorerQuickAccess::get_reserve_dir_path_str(&key) };
                cont.view.tooltip_vec = vec![cont.path.clone()];
                cont.dispnm = format!("{}{}", cont.icon, key);
                rtn_vec.push(cont.clone());
            }
        }

        quick_access.vec = rtn_vec;

        Log::debug("quick_access.vec", &quick_access.vec);

        return quick_access;
    }

    pub fn set_size(&mut self, view: &View) {
        let x = CfgEdit::get().general.activitybar.width;

        self.base.view.width = CfgEdit::get().general.sidebar.explorer.quick_access.width;
        self.base.view.y = view.y + Explorer::HEADER_HEIGHT;
        self.base.view.height = view.height;
        self.base.view.x = x;

        for (idx, cont) in self.vec.iter_mut().enumerate() {
            cont.view.width = self.base.view.width - ExplorerQuickAccess::SPLIT_LINE_WIDTH;
            cont.view.height = 1;
            cont.view.x = x;
            cont.view.y = view.y + Explorer::HEADER_HEIGHT + idx;
        }
    }

    pub fn draw(&self, str_vec: &mut Vec<String>) {
        // quick_access
        for cont in self.vec.iter() {
            str_vec.push(format!("{}", MoveTo(cont.view.x as u16, cont.view.y as u16)));
            str_vec.push(cont.dispnm.clone());
        }
        // quick_access split line
        str_vec.push(Colors::get_default_bg());
        for i in self.base.view.y..self.base.view.y_height() {
            str_vec.push(format!("{}", MoveTo(self.base.view.x_width() as u16, i as u16)));
            str_vec.push(get_space(ExplorerQuickAccess::SPLIT_LINE_WIDTH));
        }
    }

    pub fn get_reserve_dir_path_str(key: &str) -> String {
        match key {
            ExplorerQuickAccess::HOME_DIR => return Dir::get_home_dir(),
            ExplorerQuickAccess::CONFIG_DIR => return Dir::get_config_dir(),
            ExplorerQuickAccess::DESKTOP_DIR => return Dir::get_desktop_dir(),
            _ => return "".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExplorerQuickAccess {
    // pub view: View,
    pub base: SideBarContBase,
    pub vec: Vec<ExplorerQuickAccessCont>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ExplorerQuickAccessCont {
    #[serde(skip_deserializing)]
    pub view: View,
    // Includes split line
    #[serde(skip_deserializing)]
    pub dispnm: String,
    pub icon: String,
    #[serde(rename(deserialize = "path"))]
    pub path_opt: Option<String>,
    #[serde(skip_deserializing)]
    pub path: String,
}
