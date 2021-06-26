use super::keys::{KeyWhen, Keybind};
use crate::{
    _cfg::keys::{Key, KeyCmd, Keys},
    def::*,
    global::*,
    log::Log,
};
use crossterm::event::{Event::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind, *};
use directories::BaseDirs;
use json5;
use std::{collections::HashMap, fs, fs::File, io::Write, str::FromStr};

impl Keybind {
    pub fn init() -> String {
        Log::info_key("Keybind.init");

        let mut keybind_vec: Vec<Keybind> = json5::from_str(include_str!("../../keybind.json5")).unwrap();

        let mut err_str = "".to_string();

        if let Some(base_dirs) = BaseDirs::new() {
            let keybind_dir = base_dirs.config_dir();
            let keybind_file = &keybind_dir.join(env!("CARGO_PKG_NAME")).join(KEYBINDING_FILE);

            if keybind_file.exists() {
                let mut read_str = String::new();

                match fs::read_to_string(keybind_file) {
                    Ok(str) => {
                        read_str = str;
                        Log::info("read keybind.json5", &read_str);
                    }
                    Err(e) => {
                        err_str = format!("{} {} {}", LANG.file_loading_failed, keybind_file.to_string_lossy().to_string(), e);
                    }
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
                        Err(e) => {
                            err_str = format!("{} {} {}", LANG.file_parsing_failed, keybind_file.to_string_lossy().to_string(), e);
                        }
                    };
                }
            }
        }

        if cfg!(debug_assertions) {
            let mut file = File::create(KEYBINDING_FILE).unwrap();
            let s = json5::to_string(&keybind_vec).unwrap();
            write!(file, "{}", s).unwrap();
            file.flush().unwrap();
        }

        let mut key_cmd_map: HashMap<(Keys, KeyWhen), KeyCmd> = HashMap::new();
        let mut cmd_key_map: HashMap<KeyCmd, Keys> = HashMap::new();

        for keybind in keybind_vec {
            key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::from_str(&keybind.when).unwrap()), KeyCmd::from_str(&keybind.cmd).unwrap());
            cmd_key_map.insert(KeyCmd::from_str(&keybind.cmd).unwrap(), Keys::from_str(&keybind.key).unwrap());
        }

        key_cmd_map.insert((Keys::Raw(Key::Tab), KeyWhen::EditorFocus), KeyCmd::Tab);
        key_cmd_map.insert((Keys::Raw(Key::BackTab), KeyWhen::PromptFocus), KeyCmd::BackTab);

        key_cmd_map.insert((Keys::MouseScrollDown, KeyWhen::EditorFocus), KeyCmd::MouseScrollDown);
        key_cmd_map.insert((Keys::MouseScrollUp, KeyWhen::EditorFocus), KeyCmd::MouseScrollUp);
        key_cmd_map.insert((Keys::Resize, KeyWhen::EditorFocus), KeyCmd::Resize);
        key_cmd_map.insert((Keys::Unsupported, KeyWhen::EditorFocus), KeyCmd::Null);
        key_cmd_map.insert((Keys::Null, KeyWhen::EditorFocus), KeyCmd::Null);

        cmd_key_map.insert(KeyCmd::Tab, Keys::Raw(Key::Tab));
        cmd_key_map.insert(KeyCmd::MouseScrollDown, Keys::MouseScrollDown);
        cmd_key_map.insert(KeyCmd::MouseScrollUp, Keys::MouseScrollUp);
        cmd_key_map.insert(KeyCmd::Resize, Keys::Resize);
        cmd_key_map.insert(KeyCmd::Null, Keys::Unsupported);

        let _ = KEY_CMD_MAP.set(key_cmd_map);
        let _ = CMD_KEY_MAP.set(cmd_key_map);
        return err_str;
    }

    pub fn get_keycmd(keys: &Keys, keywhen: KeyWhen) -> KeyCmd {
        match &keys {
            Keys::Shift(Key::Char(c)) => return KeyCmd::InsertChar(c.to_ascii_uppercase()),
            Keys::Raw(Key::Char(c)) => return KeyCmd::InsertChar(*c),
            Keys::MouseDownLeft(y, x) => return KeyCmd::MouseDownLeft(*y as usize, *x as usize),
            Keys::MouseDragLeft(y, x) => return KeyCmd::MouseDragLeft(*y as usize, *x as usize),
            _ => {}
        };

        let result = KEY_CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::AllFocus)).or_else(|| KEY_CMD_MAP.get().unwrap().get(&(*keys, keywhen)));
        let keybindcmd = match result {
            Some(cmd) => *cmd,
            None => *KEY_CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::InputFocus)).unwrap_or(&KeyCmd::Unsupported),
        };
        return keybindcmd;
    }

    pub fn evt_to_keys(evt: &Event) -> Keys {
        match evt {
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
                match m {
                    &KeyModifiers::CONTROL => Keys::Ctrl(inner),
                    &KeyModifiers::ALT => Keys::Alt(inner),
                    &KeyModifiers::SHIFT => Keys::Shift(inner),
                    &KeyModifiers::NONE => Keys::Raw(inner),
                    _ => Keys::Unsupported,
                }
            }
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), row: y, column: x, .. }) => return Keys::MouseDownLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), row: y, column: x, .. }) => return Keys::MouseDragLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => return Keys::MouseScrollUp,
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => return Keys::MouseScrollDown,
            Resize(_, _) => return Keys::Resize,
            _ => Keys::Null,
        }
    }

    pub fn is_edit(keybindcmd: KeyCmd, is_incl_unredo: bool) -> bool {
        match keybindcmd {
            KeyCmd::InsertChar(_) | KeyCmd::Tab | KeyCmd::InsertLine | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::Paste | KeyCmd::CutSelect => return true,
            KeyCmd::Undo | KeyCmd::Redo => {
                if is_incl_unredo {
                    return true;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    pub fn get_menu_str(str: &str, cmd: KeyCmd) -> String {
        return format!("{}({})", str, Keybind::get_key_str(cmd));
    }
    pub fn get_key_str(cmd: KeyCmd) -> String {
        let key = CMD_KEY_MAP.get().unwrap().get(&cmd).unwrap();
        return key.to_string();
    }
    pub fn get_keys(keycmd: KeyCmd) -> Keys {
        return *CMD_KEY_MAP.get().unwrap().get(&(keycmd.clone())).unwrap();
    }

    pub fn check_keybind_file(keybind: &Keybind, i: usize) -> String {
        let mut msg = &String::new();
        let mut err_key = &String::new();
        let mut err_str = String::new();

        if Keys::from_str(&keybind.key).is_err() {
            msg = &LANG.specification_err_key;
            err_key = &keybind.key;
        } else if KeyCmd::from_str(&keybind.cmd).is_err() {
            msg = &LANG.specification_err_keycmd;
            err_key = &keybind.cmd;
        } else if KeyWhen::from_str(&keybind.when).is_err() {
            msg = &LANG.specification_err_keywhen;
            err_key = &keybind.when;
        }

        if !msg.is_empty() {
            err_str = format!("{} {} {} setting {}{} {}", LANG.file_parsing_failed, KEYBINDING_FILE, msg, (i + 1).to_string(), ordinal_suffix(i + 1), err_key);
        }
        return err_str;
    }
}

fn ordinal_suffix(number: usize) -> &'static str {
    match (number % 10, number % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    }
}
