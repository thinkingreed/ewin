use crate::{
    _cfg::key::{keycmd::*, keys::Key, keys::*, keywhen::*},
    global::*,
    util::*,
};
use convert_case::{Case, Casing};
use crossterm::event::{Event::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind, *};
use ewin_cfg::{
    global::*,
    lang::lang_cfg::*,
    log::*,
    model::{default::*, modal::*},
    setting_file_loader::*,
};
use ewin_const::def::*;
use json5;
use std::{cmp::Ordering::*, collections::HashMap, str::FromStr};

impl Keybind {
    pub fn init(args: &AppArgs) -> String {
        #[cfg(target_family = "unix")]
        let mut keybind_vec: Vec<Keybind> = json5::from_str(include_str!("../../../../../setting/keybind/keybind_unix_family.json5")).unwrap();
        #[cfg(target_family = "windows")]
        let mut keybind_vec: Vec<Keybind> = json5::from_str(include_str!("../../../../../setting/keybind/keybind_windows.json5")).unwrap();

        let (mut keybind_vec_user, err_str) = SettingFileLoader::new(FileType::JSON5, args, CFgFilePath::get_app_config_dir(), KEYBINDING_FILE).load::<Vec<Keybind>>();

        if !err_str.is_empty() {
            return err_str;
        }
        Cfg::write_setting_file(args, KEYBINDING_FILE, &json5::to_string(&keybind_vec).unwrap());

        for (i, keybind) in keybind_vec_user.iter().enumerate() {
            let err_str = Keybind::check_keybind_file(keybind, i);
            if !err_str.is_empty() {
                return err_str;
            }
        }
        keybind_vec.append(&mut keybind_vec_user);

        let mut key_cmd_map: HashMap<(Keys, KeyWhen), KeyCmd> = HashMap::new();
        let mut cmd_key_map: HashMap<KeyCmd, Keys> = HashMap::new();

        for keybind in keybind_vec {
            Log::debug("", &keybind);
            Log::debug("KeyCmd::cmd_when_to_keycmd", &KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &keybind.when));
            Log::debug("Keys::from_str", &Keys::from_str(&keybind.key).unwrap());

            cmd_key_map.insert(KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &keybind.when), Keys::from_str(&keybind.key).unwrap());

            if keybind.when == KeyWhen::AllFocus.to_string() {
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::AllFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::AllFocus.to_string()));
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::EditorFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::EditorFocus.to_string()));
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::PromFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::PromFocus.to_string()));
                cmd_key_map.insert(KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::EditorFocus.to_string()), Keys::from_str(&keybind.key).unwrap());
                cmd_key_map.insert(KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::PromFocus.to_string()), Keys::from_str(&keybind.key).unwrap());
            } else if keybind.when == KeyWhen::InputFocus.to_string() {
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::EditorFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::EditorFocus.to_string()));
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::PromFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::PromFocus.to_string()));
                cmd_key_map.insert(KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::EditorFocus.to_string()), Keys::from_str(&keybind.key).unwrap());
                cmd_key_map.insert(KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::PromFocus.to_string()), Keys::from_str(&keybind.key).unwrap());
            } else {
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::from_str(&keybind.when).unwrap()), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &keybind.when));
            }
        }

        // cmd_key_map.insert(KeyCmd::Edit(E_Cmd::InsertStr("".to_string())), Keys::Ctrl(Key::Char('v')));
        Log::debug("cmd_key_map", &cmd_key_map);

        // Editor
        key_cmd_map.insert((Keys::Raw(Key::Tab), KeyWhen::EditorFocus), KeyCmd::Edit(E_Cmd::InsertStr(TAB_CHAR.to_string())));
        key_cmd_map.insert((Keys::MouseScrollDown, KeyWhen::EditorFocus), KeyCmd::Edit(E_Cmd::MouseScrollDown));
        key_cmd_map.insert((Keys::MouseScrollUp, KeyWhen::EditorFocus), KeyCmd::Edit(E_Cmd::MouseScrollUp));
        key_cmd_map.insert((Keys::Null, KeyWhen::EditorFocus), KeyCmd::Edit(E_Cmd::Null));

        // Prom
        key_cmd_map.insert((Keys::MouseScrollDown, KeyWhen::PromFocus), KeyCmd::Prom(P_Cmd::MouseScrollDown));
        key_cmd_map.insert((Keys::MouseScrollUp, KeyWhen::PromFocus), KeyCmd::Prom(P_Cmd::MouseScrollUp));
        key_cmd_map.insert((Keys::Null, KeyWhen::PromFocus), KeyCmd::Prom(P_Cmd::Null));

        // MenuWidget
        key_cmd_map.insert((Keys::Null, KeyWhen::MenuBarFocus), KeyCmd::MenuBar(M_Cmd::Null));
        key_cmd_map.insert((Keys::Raw(Key::Left), KeyWhen::MenuBarFocus), KeyCmd::MenuBar(M_Cmd::CursorLeft));
        key_cmd_map.insert((Keys::Raw(Key::Right), KeyWhen::MenuBarFocus), KeyCmd::MenuBar(M_Cmd::CursorRight));
        key_cmd_map.insert((Keys::Raw(Key::Up), KeyWhen::MenuBarFocus), KeyCmd::MenuBar(M_Cmd::CursorUp));
        key_cmd_map.insert((Keys::Raw(Key::Down), KeyWhen::MenuBarFocus), KeyCmd::MenuBar(M_Cmd::CursorDown));
        key_cmd_map.insert((Keys::Raw(Key::Enter), KeyWhen::MenuBarFocus), KeyCmd::MenuBar(M_Cmd::Confirm));

        // CtxMenu
        key_cmd_map.insert((Keys::Null, KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::Null));
        key_cmd_map.insert((Keys::Raw(Key::Left), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::CursorLeft));
        key_cmd_map.insert((Keys::Raw(Key::Right), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::CursorRight));
        key_cmd_map.insert((Keys::Raw(Key::Up), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::CursorUp));
        key_cmd_map.insert((Keys::Raw(Key::Down), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::CursorDown));
        key_cmd_map.insert((Keys::Raw(Key::Enter), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::Confirm));

        Log::debug("key_cmd_map", &key_cmd_map);

        let _ = CMD_KEY_MAP.set(cmd_key_map);

        let _ = KEY_CMD_MAP.set(key_cmd_map);
        err_str
    }

    //  pub fn keys_to_keycmd_pressed(keys: &Keys, keys_org_opt: Option<&Keys>, keywhen: KeyWhen, hbar_row_posi: usize, sbar_row_posi: usize, scrl_v_is_enable: bool, scrl_h_is_enable: bool) -> KeyCmd {
    pub fn keys_to_keycmd_pressed(keys: &Keys, keys_org_opt: Option<&Keys>, keywhen: KeyWhen) -> KeyCmd {
        Log::debug_key("keys_to_keycmd_overall");
        Log::debug("keys", &keys);
        Log::debug("keywhen", &keywhen);

        //  let result = KEY_CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::AllFocus)).or_else(|| KEY_CMD_MAP.get().unwrap().get(&(*keys, keywhen.clone())));
        let result = KEY_CMD_MAP.get().unwrap().get(&(*keys, keywhen.clone()));
        let keycmd = match result {
            Some(keycmd) => keycmd.clone(),
            None => KEY_CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::InputFocus)).unwrap_or(&KeyCmd::Unsupported).clone(),
        };
        Log::debug("keycmd", &keycmd);

        if keycmd != KeyCmd::Unsupported {
            return keycmd;
        }
        match keywhen {
            KeyWhen::MenuBarFocus => {
                match &keys {
                    Keys::MouseDownLeft(y, x) => return KeyCmd::MenuBar(M_Cmd::MouseDownLeft(*y as usize, *x as usize)),
                    Keys::MouseMove(y, x) => return KeyCmd::MenuBar(M_Cmd::MouseMove(*y as usize, *x as usize)),

                    _ => {}
                };
                return KeyCmd::Unsupported;
            }
            KeyWhen::FileBarFocus => {
                match &keys {
                    Keys::MouseDownLeft(y, x) => return KeyCmd::FileBar(F_Cmd::MouseDownLeft(*y as usize, *x as usize)),
                    Keys::MouseUpLeft(y, x) => return KeyCmd::FileBar(F_Cmd::MouseUpLeft(*y as usize, *x as usize)),
                    Keys::MouseDragLeft(y, x) => {
                        match keys_org_opt {
                            Some(Keys::MouseDragLeft(y_org, x_org)) | Some(Keys::MouseDownLeft(y_org, x_org)) => {
                                return match y.cmp(y_org) {
                                    Less => KeyCmd::FileBar(F_Cmd::MouseDragLeftUp(*y as usize, *x as usize)),
                                    Greater => KeyCmd::FileBar(F_Cmd::MouseDragLeftDown(*y as usize, *x as usize)),
                                    Equal => {
                                        if x > x_org || x == &(get_term_size().0 as u16) {
                                            KeyCmd::FileBar(F_Cmd::MouseDragLeftRight(*y as usize, *x as usize))
                                        } else {
                                            KeyCmd::FileBar(F_Cmd::MouseDragLeftLeft(*y as usize, *x as usize))
                                        }
                                    }
                                };
                            }
                            _ => {}
                        };
                    }
                    _ => {}
                };
                return KeyCmd::Unsupported;
            }
            KeyWhen::StatusBarFocus => {
                if let Keys::MouseDownLeft(y, x) = &keys {
                    return KeyCmd::StatusBar(S_Cmd::MouseDownLeft(*y as usize, *x as usize));
                }
                return KeyCmd::Unsupported;
            }
            KeyWhen::EditorFocus => {
                match &keys {
                    Keys::Resize(x, y) => return KeyCmd::Edit(E_Cmd::Resize(*x as usize, *y as usize)),
                    Keys::Shift(Key::Char(c)) => return KeyCmd::Edit(E_Cmd::InsertStr(c.to_ascii_uppercase().to_string())),
                    Keys::Raw(Key::Char(c)) => return KeyCmd::Edit(E_Cmd::InsertStr(c.to_string())),
                    Keys::MouseAltDownLeft(y, x) => return KeyCmd::Edit(E_Cmd::MouseDownLeftBox(*y as usize, *x as usize)),
                    Keys::MouseAltDragLeft(y, x) => return KeyCmd::Edit(E_Cmd::MouseDragLeftBox(*y as usize, *x as usize)),
                    Keys::MouseDownLeft(y, x) => return KeyCmd::Edit(E_Cmd::MouseDownLeft(*y as usize, *x as usize)),
                    Keys::MouseUpLeft(y, x) => return KeyCmd::Edit(E_Cmd::MouseUpLeft(*y as usize, *x as usize)),
                    Keys::MouseDragLeft(y, x) => {
                        match keys_org_opt {
                            Some(Keys::MouseDragLeft(y_org, x_org)) | Some(Keys::MouseDownLeft(y_org, x_org)) => {
                                return match y.cmp(y_org) {
                                    Less => KeyCmd::Edit(E_Cmd::MouseDragLeftUp(*y as usize, *x as usize)),
                                    Greater => KeyCmd::Edit(E_Cmd::MouseDragLeftDown(*y as usize, *x as usize)),
                                    Equal => {
                                        if x > x_org || x == &(get_term_size().0 as u16) {
                                            KeyCmd::Edit(E_Cmd::MouseDragLeftRight(*y as usize, *x as usize))
                                        } else {
                                            KeyCmd::Edit(E_Cmd::MouseDragLeftLeft(*y as usize, *x as usize))
                                        }
                                    }
                                };
                            }
                            _ => {}
                        };
                    }
                    Keys::MouseDownRight(y, x) | Keys::MouseDragRight(y, x) => return KeyCmd::Edit(E_Cmd::CtxtMenu(*y as usize, *x as usize)),
                    Keys::MouseMove(y, x) => return KeyCmd::Edit(E_Cmd::MouseMove(*y as usize, *x as usize)),
                    _ => {}
                };
                return KeyCmd::Unsupported;
            }
            KeyWhen::PromFocus => {
                let p_cmd = match &keys {
                    Keys::Resize(x, y) => P_Cmd::Resize(*x as usize, *y as usize),
                    Keys::Shift(Key::Char(c)) => P_Cmd::InsertStr(c.to_ascii_uppercase().to_string()),
                    Keys::Raw(Key::Char(c)) => P_Cmd::InsertStr(c.to_string()),
                    Keys::MouseDownLeft(y, x) => P_Cmd::MouseDownLeft(*y as usize, *x as usize),
                    Keys::MouseDragLeft(y, x) => P_Cmd::MouseDragLeft(*y as usize, *x as usize),
                    Keys::MouseMove(y, x) => P_Cmd::MouseMove(*y as usize, *x as usize),
                    _ => return KeyCmd::Unsupported,
                };
                return KeyCmd::Prom(p_cmd);
            }
            KeyWhen::CtxMenuFocus => {
                let c_cmd = match &keys {
                    Keys::MouseDownLeft(y, x) => C_Cmd::MouseDownLeft(*y as usize, *x as usize),
                    Keys::MouseDownRight(y, x) => C_Cmd::CtxMenu(*y as usize, *x as usize),
                    Keys::MouseDragRight(_, _) => C_Cmd::Null,
                    Keys::MouseMove(y, x) => C_Cmd::MouseMove(*y as usize, *x as usize),
                    _ => return KeyCmd::Unsupported,
                };
                return KeyCmd::CtxMenu(c_cmd);
            }
            _ => return KeyCmd::Unsupported,
        };
    }

    pub fn evt_to_keys(evt: &Event) -> Keys {
        return match evt {
            Event::Key(KeyEvent { code: c, modifiers: m }) => {
                let inner = match c {
                    KeyCode::Char(c) => Key::Char(*c),
                    KeyCode::BackTab => Key::BackTab,
                    KeyCode::Insert => Key::Insert,
                    KeyCode::Esc => Key::Esc,
                    KeyCode::Backspace => Key::Backspace,
                    KeyCode::Tab => Key::Tab,
                    KeyCode::Enter => Key::Enter,
                    KeyCode::Delete => Key::Delete,
                    KeyCode::Null => Key::Null,
                    KeyCode::PageUp => Key::PageUp,
                    KeyCode::PageDown => Key::PageDown,
                    KeyCode::Home => Key::Home,
                    KeyCode::End => Key::End,
                    KeyCode::Up => Key::Up,
                    KeyCode::Down => Key::Down,
                    KeyCode::Left => Key::Left,
                    KeyCode::Right => Key::Right,
                    KeyCode::F(i) => Key::F(*i),
                };
                match *m {
                    KeyModifiers::CONTROL => Keys::Ctrl(inner),
                    KeyModifiers::ALT => Keys::Alt(inner),
                    KeyModifiers::SHIFT => Keys::Shift(inner),
                    KeyModifiers::NONE => Keys::Raw(inner),
                    _ => Keys::Unsupported,
                }
            }
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), modifiers: KeyModifiers::ALT, row: y, column: x, .. }) => Keys::MouseAltDownLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), modifiers: KeyModifiers::ALT, row: y, column: x, .. }) => Keys::MouseAltDragLeft(*y, *x),
            // Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), modifiers: KeyModifiers::ALT, .. }) => return Keys::MouseAltUpLeft,
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), row: y, column: x, .. }) => Keys::MouseDownLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), row: y, column: x, .. }) => Keys::MouseUpLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Right), row: y, column: x, .. }) => Keys::MouseDownRight(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), row: y, column: x, .. }) => Keys::MouseDragLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Right), row: y, column: x, .. }) => Keys::MouseDragRight(*y, *x),
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => Keys::MouseScrollUp,
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => Keys::MouseScrollDown,
            Mouse(M_Event { kind: M_Kind::Moved, row: y, column: x, .. }) => Keys::MouseMove(*y, *x),
            Resize(x, y) => Keys::Resize(*x, *y),
            _ => Keys::Null,
        };
    }

    pub fn get_edit_func_str(funcnm: &str) -> Option<String> {
        Log::debug_key("get_edit_func_str");
        Log::debug("funcnm", &funcnm);
        if let Some(name) = LANG_MAP.get(&funcnm.to_case(Case::Snake)) {
            Log::debug("name", &name);
            let cmd = KeyCmd::to_edit_cmd(funcnm);
            Log::debug("cmd", &cmd);
            if cmd == E_Cmd::Null {
                Some(name.clone())
            } else {
                Some(Keybind::get_menu_str(name, KeyCmd::Edit(cmd)))
            }
        } else {
            None
        }
    }

    pub fn get_menu_str(menunm: &str, cmd: KeyCmd) -> String {
        let str = Keybind::get_key_str(cmd);
        let key_str = if str.is_empty() { "".to_string() } else { format!("({})", str) };
        return format!("{}{}", menunm, key_str);
    }

    pub fn get_key_str(cmd: KeyCmd) -> String {
        let result = CMD_KEY_MAP.get().unwrap().get(&cmd);
        match result {
            Some(key) => key.to_string(),
            None => "".to_string(),
        }
    }
    pub fn keycmd_to_keys(keycmd: &KeyCmd) -> Keys {
        *CMD_KEY_MAP.get().unwrap().get(keycmd).unwrap()
    }

    pub fn check_keybind_file(keybind: &Keybind, i: usize) -> String {
        let mut msg = &String::new();
        let mut err_key = &String::new();
        let mut err_str = String::new();

        if Keys::from_str(&keybind.key).is_err() {
            msg = &Lang::get().specification_err_key;
            err_key = &keybind.key;
        } else if KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &keybind.when) == KeyCmd::Unsupported {
            msg = &Lang::get().specification_err_keycmd;
            err_key = &keybind.cmd;
        } else if KeyWhen::from_str(&keybind.when).is_err() {
            msg = &Lang::get().specification_err_keywhen;
            err_key = &keybind.when;
        }

        if !msg.is_empty() {
            err_str = format!("{}{} {} {} setting {}{} {}", Lang::get().file, Lang::get().parsing_failed, KEYBINDING_FILE, msg, (i + 1), ordinal_suffix(i + 1), err_key);
        }
        err_str
    }
}
