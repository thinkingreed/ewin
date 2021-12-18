use crate::{
    _cfg::{
        key::{keycmd::*, keys::Key, keys::*, keywhen::*},
        lang::lang_cfg::*,
    },
    def::*,
    global::*,
    log::*,
    model::*,
    util::*,
};
use crossterm::event::{Event::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind, *};
use directories::BaseDirs;
use json5;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    str::FromStr,
};

impl Keybind {
    pub fn init(args: &Args, keybind_str: &str) -> String {
        let mut keybind_vec: Vec<Keybind> = json5::from_str(keybind_str).unwrap();

        let mut err_str = "".to_string();

        if let Some(base_dirs) = BaseDirs::new() {
            let keybind_file = &base_dirs.config_dir().join(APP_NAME).join(KEYBINDING_FILE);

            if keybind_file.exists() {
                let mut read_str = String::new();

                match fs::read_to_string(keybind_file) {
                    Ok(str) => {
                        read_str = str;
                        Log::info("read keybind.json5", &read_str);
                    }
                    Err(e) => err_str = format!("{} {} {}", Lang::get().file_loading_failed, keybind_file.to_string_lossy().to_string(), e),
                }

                if err_str.is_empty() {
                    match json5::from_str(&read_str) {
                        Ok(vec) => {
                            keybind_vec = vec;
                            for (i, keybind) in keybind_vec.iter().enumerate() {
                                let err_str = Keybind::check_keybind_file(keybind, i);
                                if !err_str.is_empty() {
                                    return err_str;
                                }
                            }
                        }
                        Err(e) => err_str = format!("{}{} {} {}", Lang::get().file, Lang::get().parsing_failed, keybind_file.to_string_lossy().to_string(), e),
                    };
                }
            } else if args.out_config_flg {
                if let Ok(mut file) = File::create(keybind_file) {
                    let _ = write!(&mut file, "{}", keybind_str);
                    let _ = &mut file.flush().unwrap();
                }
            }
        }

        let mut key_cmd_map: HashMap<(Keys, KeyWhen), KeyCmd> = HashMap::new();
        let mut cmd_key_map: HashMap<KeyCmd, Keys> = HashMap::new();

        for keybind in keybind_vec {
            Log::debug("keybind", &keybind);
            cmd_key_map.insert(KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &keybind.when), Keys::from_str(&keybind.key).unwrap());

            if keybind.when == KeyWhen::InputFocus.to_string() || keybind.when == KeyWhen::AllFocus.to_string() {
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::EditorFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::EditorFocus.to_string()));
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::PromptFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::PromptFocus.to_string()));
                cmd_key_map.insert(KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::EditorFocus.to_string()), Keys::from_str(&keybind.key).unwrap());
                cmd_key_map.insert(KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::PromptFocus.to_string()), Keys::from_str(&keybind.key).unwrap());
            //  } else if keybind.when == KeyWhen::AllFocus.to_string() {
            //      key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::EditorFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::EditorFocus.to_string()));
            //      key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::PromptFocus), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &KeyWhen::PromptFocus.to_string()));
            } else {
                key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::from_str(&keybind.when).unwrap()), KeyCmd::cmd_when_to_keycmd(&keybind.cmd, &keybind.when));
            }
        }

        Log::debug("cmd_key_map", &cmd_key_map);

        key_cmd_map.insert((Keys::Raw(Key::Tab), KeyWhen::EditorFocus), KeyCmd::Edit(E_Cmd::InsertStr(TAB_CHAR.to_string())));
        key_cmd_map.insert((Keys::Raw(Key::Tab), KeyWhen::PromptFocus), KeyCmd::Prom(P_Cmd::TabNextFocus));
        key_cmd_map.insert((Keys::Shift(Key::BackTab), KeyWhen::PromptFocus), KeyCmd::Prom(P_Cmd::BackTabBackFocus));

        key_cmd_map.insert((Keys::MouseScrollDown, KeyWhen::EditorFocus), KeyCmd::Edit(E_Cmd::MouseScrollDown));
        key_cmd_map.insert((Keys::MouseScrollDown, KeyWhen::PromptFocus), KeyCmd::Prom(P_Cmd::MouseScrollDown));
        key_cmd_map.insert((Keys::MouseScrollUp, KeyWhen::EditorFocus), KeyCmd::Edit(E_Cmd::MouseScrollUp));
        key_cmd_map.insert((Keys::MouseScrollUp, KeyWhen::PromptFocus), KeyCmd::Prom(P_Cmd::MouseScrollUp));
        key_cmd_map.insert((Keys::Null, KeyWhen::EditorFocus), KeyCmd::Edit(E_Cmd::Null));
        key_cmd_map.insert((Keys::Null, KeyWhen::PromptFocus), KeyCmd::Prom(P_Cmd::Null));

        // CtxMenu
        key_cmd_map.insert((Keys::Null, KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::Null));
        key_cmd_map.insert((Keys::Raw(Key::Left), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::CursorLeft));
        key_cmd_map.insert((Keys::Raw(Key::Right), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::CursorRight));
        key_cmd_map.insert((Keys::Raw(Key::Up), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::CursorUp));
        key_cmd_map.insert((Keys::Raw(Key::Down), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::CursorDown));
        key_cmd_map.insert((Keys::Raw(Key::Enter), KeyWhen::CtxMenuFocus), KeyCmd::CtxMenu(C_Cmd::ConfirmCtxMenu));

        Log::debug("key_cmd_map", &key_cmd_map);

        let _ = KEY_CMD_MAP.set(key_cmd_map);
        let _ = CMD_KEY_MAP.set(cmd_key_map);
        err_str
    }

    pub fn keys_to_keycmd(keys: &Keys, keywhen: KeyWhen) -> KeyCmd {
        return Keybind::keys_to_keycmd_pressed(keys, None, keywhen, USIZE_UNDEFINED, USIZE_UNDEFINED);
    }

    pub fn keys_to_keycmd_pressed(keys: &Keys, keys_org_opt: Option<&Keys>, keywhen: KeyWhen, hbar_row_posi: usize, sbar_row_posi: usize) -> KeyCmd {
        Log::debug_key("keys_to_keycmd_overall");
        Log::debug("keys", &keys);

        let result = KEY_CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::AllFocus)).or_else(|| KEY_CMD_MAP.get().unwrap().get(&(*keys, keywhen.clone())));
        let keycmd = match result {
            Some(cmd) => cmd.clone(),
            None => KEY_CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::InputFocus)).unwrap_or(&KeyCmd::Unsupported).clone(),
        };
        Log::debug("keycmd", &keycmd);

        if keycmd != KeyCmd::Unsupported {
            return keycmd;
        }
        if keys == &Keys::Resize {
            return KeyCmd::Resize;
        }

        match keywhen {
            KeyWhen::HeaderBarFocus => {
                if let Keys::MouseDownLeft(y, x) = &keys {
                    return KeyCmd::HeaderBar(H_Cmd::MouseDownLeft(*y as usize, *x as usize));
                }

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
                    Keys::Shift(Key::Char(c)) => return KeyCmd::Edit(E_Cmd::InsertStr(c.to_ascii_uppercase().to_string())),
                    Keys::Raw(Key::Char(c)) => return KeyCmd::Edit(E_Cmd::InsertStr(c.to_string())),
                    Keys::MouseAltDownLeft(y, x) => return KeyCmd::Edit(E_Cmd::MouseDownLeftBox(*y as usize, *x as usize)),
                    Keys::MouseAltDragLeft(y, x) => return KeyCmd::Edit(E_Cmd::MouseDragLeftBox(*y as usize, *x as usize)),
                    Keys::MouseDownLeft(y, x) => return KeyCmd::Edit(E_Cmd::MouseDownLeft(*y as usize, *x as usize)),
                    Keys::MouseDragLeft(y, x) => {
                        match keys_org_opt {
                            Some(Keys::MouseDragLeft(y_org, x_org)) | Some(Keys::MouseDownLeft(y_org, x_org)) => {
                                let cols = get_term_size().0 as usize;
                                return if y < y_org || (y == &(hbar_row_posi as u16) && x < &((cols - CFG.get().unwrap().try_lock().unwrap().general.editor.scrollbar.vertical.width) as u16)) {
                                    KeyCmd::Edit(E_Cmd::MouseDragLeftUp(*y as usize, *x as usize))
                                } else if y > y_org || (y == &(sbar_row_posi as u16) && x < &((cols - CFG.get().unwrap().try_lock().unwrap().general.editor.scrollbar.vertical.width) as u16)) {
                                    KeyCmd::Edit(E_Cmd::MouseDragLeftDown(*y as usize, *x as usize))
                                } else if y == y_org {
                                    if x > x_org || x == &(cols as u16) {
                                        KeyCmd::Edit(E_Cmd::MouseDragLeftRight(*y as usize, *x as usize))
                                    } else {
                                        KeyCmd::Edit(E_Cmd::MouseDragLeftLeft(*y as usize, *x as usize))
                                    }
                                } else {
                                    //     KeyCmd::Edit(E_Cmd::MouseDragLeftDown(*y as usize, *x as usize))
                                    return KeyCmd::Unsupported;
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
            KeyWhen::PromptFocus => {
                let p_cmd = match &keys {
                    Keys::Shift(Key::Char(c)) => P_Cmd::InsertStr(c.to_ascii_uppercase().to_string()),
                    Keys::Raw(Key::Char(c)) => P_Cmd::InsertStr(c.to_string()),
                    //  Keys::Raw(Key::F(3)) => P_Cmd::FindNext,
                    //  Keys::Shift(Key::F(4)) => P_Cmd::FindBack,
                    Keys::MouseDownLeft(y, x) => P_Cmd::MouseDownLeft(*y as usize, *x as usize),
                    Keys::MouseDragLeft(y, x) => P_Cmd::MouseDragLeft(*y as usize, *x as usize),
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
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Right), row: y, column: x, .. }) => Keys::MouseDownRight(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), row: y, column: x, .. }) => Keys::MouseDragLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Right), row: y, column: x, .. }) => Keys::MouseDragRight(*y, *x),
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => Keys::MouseScrollUp,
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => Keys::MouseScrollDown,
            Mouse(M_Event { kind: M_Kind::Moved, row: y, column: x, .. }) => Keys::MouseMove(*y, *x),
            Resize(_, _) => Keys::Resize,
            _ => Keys::Null,
        };
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
            err_str = format!("{}{} {} {} setting {}{} {}", Lang::get().file, Lang::get().parsing_failed, KEYBINDING_FILE, msg, (i + 1).to_string(), ordinal_suffix(i + 1), err_key);
        }
        err_str
    }
}
