use crate::{bar::msgbar::*, bar::statusbar::*, def::*, global::*, log::*, model::*, prompt::prompt::*, terminal::*, util::*};
use std::{env, ffi::OsStr, fmt, fs::metadata, io::ErrorKind, path::Path};

#[derive(Debug, Clone)]
pub struct Tab {
    pub editor: Editor,
    pub mbar: MsgBar,
    pub prom: Prompt,
    pub sbar: StatusBar,
    pub state: TabState,
}
#[derive(Debug, Clone)]
pub struct TabState {
    pub is_close_confirm: bool,
    pub is_search: bool,
    pub is_replace: bool,
    pub is_save_new_file: bool,
    pub is_move_line: bool,
    pub is_key_record: bool,
    pub is_key_record_exec: bool,
    pub is_key_record_exec_draw: bool,
    pub is_read_only: bool,
    pub grep_info: GrepInfo,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            is_close_confirm: false,
            is_search: false,
            is_replace: false,
            is_save_new_file: false,
            is_move_line: false,
            is_key_record: false,
            is_key_record_exec: false,
            is_key_record_exec_draw: false,
            is_read_only: false,
            grep_info: GrepInfo::default(),
        }
    }
}

impl TabState {
    pub fn clear(&mut self) {
        self.is_close_confirm = false;
        self.is_search = false;
        self.is_replace = false;
        self.is_save_new_file = false;
        self.is_move_line = false;
        self.grep_info.clear();
    }
}

impl fmt::Display for TabState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabState is_search:{:?},", self.is_search)
    }
}

impl Tab {
    pub fn new() -> Self {
        Tab {
            editor: Editor::new(),
            mbar: MsgBar::new(),
            prom: Prompt::new(),
            sbar: StatusBar::new(),
            state: TabState::default(),
        }
    }

    pub fn open(&mut self, filenm: &str) {
        Log::ep_s("           open");
        let path = Path::new(&filenm);

        if path.to_string_lossy().to_string().len() > 0 {
            if path.exists() {
                let file_meta = metadata(path).unwrap();
                if file_meta.permissions().readonly() {
                    self.state.is_read_only = true;
                    self.mbar.set_readonly(&format!("{}({})", &LANG.unable_to_edit, &LANG.no_write_permission));
                }
                let ext = path.extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();

                self.editor.draw.syntax_reference = if let Some(sr) = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext) { Some(sr.clone()) } else { None };
                if self.editor.draw.syntax_reference.is_some() && file_meta.len() < ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE && is_enable_syntax_highlight(&ext) {
                    self.editor.is_enable_syntax_highlight = true;
                }
            } else {
                Terminal::exit();
                println!("{}", LANG.file_not_found.clone());
            }
        } else {
            let curt_dir = env::current_dir().unwrap();
            let curt_dir = metadata(curt_dir).unwrap();
            if curt_dir.permissions().readonly() {
                Terminal::exit();
                println!("{}", LANG.no_write_permission.clone());
            }
        }
        // read
        let result = TextBuffer::from_path(&path.to_string_lossy().to_string());
        match result {
            Ok(t_buf) => {
                self.editor.buf = t_buf;
                self.editor.buf.text.insert_char(self.editor.buf.text.len_chars(), EOF_MARK);
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    Terminal::exit();
                    println!("{}", LANG.no_read_permission.clone());
                }
                ErrorKind::NotFound => self.editor.buf.text.insert_char(self.editor.buf.text.len_chars(), EOF_MARK),
                _ => {
                    Terminal::exit();
                    println!("{} {:?}", LANG.file_opening_problem, err);
                }
            },
        }
        self.editor.set_cur_default();
    }
}
