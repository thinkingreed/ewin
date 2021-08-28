use crate::{
    _cfg::keys::{Key, *},
    def::*,
    global::*,
    log::Log,
    model::Args,
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
    pub fn init(args: &Args) -> String {
        let keybind_str = include_str!("../../../keybind.json5");
        let mut keybind_vec: Vec<Keybind> = json5::from_str(keybind_str).unwrap();

        let mut err_str = "".to_string();

        if let Some(base_dirs) = BaseDirs::new() {
            let keybind_file = &base_dirs.config_dir().join(env!("CARGO_PKG_NAME")).join(KEYBINDING_FILE);

            if keybind_file.exists() {
                let mut read_str = String::new();

                match fs::read_to_string(keybind_file) {
                    Ok(str) => {
                        read_str = str;
                        Log::info("read keybind.json5", &read_str);
                    }
                    Err(e) => err_str = format!("{} {} {}", LANG.file_loading_failed, keybind_file.to_string_lossy().to_string(), e),
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
                        Err(e) => err_str = format!("{}{} {} {}", LANG.file, LANG.parsing_failed, keybind_file.to_string_lossy().to_string(), e),
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
            key_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), KeyWhen::from_str(&keybind.when).unwrap()), KeyCmd::from_str(&keybind.cmd).unwrap());
            cmd_key_map.insert(KeyCmd::from_str(&keybind.cmd).unwrap(), Keys::from_str(&keybind.key).unwrap());
        }

        key_cmd_map.insert((Keys::Raw(Key::Tab), KeyWhen::EditorFocus), KeyCmd::InsertStr(TAB_CHAR.to_string()));
        key_cmd_map.insert((Keys::Raw(Key::Tab), KeyWhen::PromptFocus), KeyCmd::Tab);
        key_cmd_map.insert((Keys::Shift(Key::BackTab), KeyWhen::PromptFocus), KeyCmd::BackTab);

        key_cmd_map.insert((Keys::MouseScrollDown, KeyWhen::AllFocus), KeyCmd::MouseScrollDown);
        key_cmd_map.insert((Keys::MouseScrollUp, KeyWhen::AllFocus), KeyCmd::MouseScrollUp);
        key_cmd_map.insert((Keys::Resize, KeyWhen::AllFocus), KeyCmd::Resize);
        key_cmd_map.insert((Keys::Unsupported, KeyWhen::EditorFocus), KeyCmd::Null);
        key_cmd_map.insert((Keys::Null, KeyWhen::EditorFocus), KeyCmd::Null);

        // For key display etc
        cmd_key_map.insert(KeyCmd::Tab, Keys::Raw(Key::Tab));
        cmd_key_map.insert(KeyCmd::MouseScrollDown, Keys::MouseScrollDown);
        cmd_key_map.insert(KeyCmd::MouseScrollUp, Keys::MouseScrollUp);
        cmd_key_map.insert(KeyCmd::Resize, Keys::Resize);
        cmd_key_map.insert(KeyCmd::Null, Keys::Unsupported);

        let _ = KEY_CMD_MAP.set(key_cmd_map);
        let _ = CMD_KEY_MAP.set(cmd_key_map);
        return err_str;
    }

    pub fn keys_to_keycmd(keys: &Keys, keywhen: KeyWhen) -> KeyCmd {
        match &keys {
            Keys::Shift(Key::Char(c)) => return KeyCmd::InsertStr(c.to_ascii_uppercase().to_string()),
            Keys::Raw(Key::Char(c)) => return KeyCmd::InsertStr(c.to_string()),
            Keys::MouseAltDownLeft(y, x) => return KeyCmd::MouseDownBoxLeft(*y as usize, *x as usize),
            Keys::MouseAltDragLeft(y, x) => return KeyCmd::MouseDragBoxLeft(*y as usize, *x as usize),
            Keys::MouseDownLeft(y, x) => return KeyCmd::MouseDownLeft(*y as usize, *x as usize),
            Keys::MouseDownRight(y, x) => return KeyCmd::MouseDownRight(*y as usize, *x as usize),
            Keys::MouseDragLeft(y, x) => return KeyCmd::MouseDragLeft(*y as usize, *x as usize),
            Keys::MouseDragRight(y, x) => return KeyCmd::MouseDragRight(*y as usize, *x as usize),
            Keys::MouseMove(y, x) => return KeyCmd::MouseMove(*y as usize, *x as usize),
            _ => {}
        };

        let result = KEY_CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::AllFocus)).or_else(|| KEY_CMD_MAP.get().unwrap().get(&(*keys, keywhen)));
        let keybindcmd = match result {
            Some(cmd) => cmd.clone(),
            None => KEY_CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::InputFocus)).unwrap_or(&KeyCmd::Unsupported).clone(),
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
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), modifiers: KeyModifiers::ALT, row: y, column: x, .. }) => return Keys::MouseAltDownLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), modifiers: KeyModifiers::ALT, row: y, column: x, .. }) => return Keys::MouseAltDragLeft(*y, *x),
            // Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), modifiers: KeyModifiers::ALT, .. }) => return Keys::MouseAltUpLeft,
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), row: y, column: x, .. }) => return Keys::MouseDownLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Right), row: y, column: x, .. }) => return Keys::MouseDownRight(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), row: y, column: x, .. }) => return Keys::MouseDragLeft(*y, *x),
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Right), row: y, column: x, .. }) => return Keys::MouseDragRight(*y, *x),
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => return Keys::MouseScrollUp,
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => return Keys::MouseScrollDown,
            Mouse(M_Event { kind: M_Kind::Moved, row: y, column: x, .. }) => return Keys::MouseMove(*y, *x),
            Resize(_, _) => return Keys::Resize,
            _ => Keys::Null,
        }
    }

    pub fn is_edit(keybindcmd: &KeyCmd, is_incl_unredo: bool) -> bool {
        match keybindcmd {
            KeyCmd::InsertStr(_) | KeyCmd::Tab | KeyCmd::InsertLine | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::Cut => return true,
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
    pub fn get_menu_str(menunm: &str, cmd: KeyCmd) -> String {
        let str = Keybind::get_key_str(cmd);
        let key_str = if str.is_empty() { "".to_string() } else { format!("({})", str) };
        return format!("{}{}", menunm, key_str);
    }
    pub fn get_key_str(cmd: KeyCmd) -> String {
        let result = CMD_KEY_MAP.get().unwrap().get(&cmd);
        return match result {
            Some(key) => key.to_string(),
            None => "".to_string(),
        };
    }
    pub fn keycmd_to_keys(keycmd: &KeyCmd) -> Keys {
        return *CMD_KEY_MAP.get().unwrap().get(&(&keycmd)).unwrap();
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
            err_str = format!("{}{} {} {} setting {}{} {}", LANG.file, LANG.parsing_failed, KEYBINDING_FILE, msg, (i + 1).to_string(), ordinal_suffix(i + 1), err_key);
        }
        return err_str;
    }
}
