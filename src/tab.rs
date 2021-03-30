use crate::{bar::msgbar::*, bar::statusbar::*, def::*, global::*, log::*, model::*, prompt::prompt::*, terminal::*, util::*};

use std::{env, fmt, fs::metadata, io::ErrorKind, path};

#[derive(Debug, Clone)]
pub struct Tab {
    pub state: TabState,
    pub editor: Editor,
    pub mbar: MsgBar,
    pub prom: Prompt,
    pub sbar: StatusBar,
}
#[derive(Debug, Clone)]
pub struct TabState {
    pub is_search: bool,
    pub is_replace: bool,
    pub grep_info: GrepInfo,
    // pub is_grep: bool,
    // pub is_grep_result: bool,
    // pub is_grep_result_cancel: bool,
    // pub is_grep_stdout_end: bool,
    // pub is_grep_stderr_end: bool,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            is_search: false,
            is_replace: false,
            grep_info: GrepInfo::default(),
            // is_grep: false,
            // is_grep_result: false,
            // is_grep_result_cancel: false,
            // is_grep_stdout_end: false,
            // is_grep_stderr_end: false,
        }
    }
}
impl TabState {
    pub fn clear(&mut self) {
        self.is_search = false;
        //  self.is_grep = false;
        self.is_replace = false;
        // self.is_grep_result = false;
        // self.is_grep_result_cancel = false;
        // self.is_grep_stdout_end = false;
        // self.is_grep_stderr_end = false;
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

    pub fn open(&mut self, path: &path::Path) {
        Log::ep_s("           open");

        if path.to_string_lossy().to_string().len() > 0 {
            if path.exists() {
                let file_meta = metadata(path).unwrap();
                if file_meta.permissions().readonly() {
                    self.mbar.set_readonly(&format!("{}({})", &LANG.unable_to_edit, &LANG.no_write_permission));
                }
                if CFG.get().unwrap().try_lock().unwrap().syntax.syntax_reference.is_some() && file_meta.len() < ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE && is_enable_syntax_highlight(&FILE.get().unwrap().try_lock().unwrap().ext) {
                    FILE.get().unwrap().try_lock().map(|mut file| file.is_enable_syntax_highlight = true).unwrap();
                }
                FILE.get().unwrap().try_lock().map(|mut file| file.path = Some(path.into())).unwrap();
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
                ErrorKind::NotFound => {
                    self.editor.buf.text.insert_char(self.editor.buf.text.len_chars(), EOF_MARK);
                    FILE.get().unwrap().try_lock().map(|mut file| file.path = None).unwrap();
                }
                _ => {
                    Terminal::exit();
                    println!("{} {:?}", LANG.file_opening_problem, err);
                }
            },
        }
        self.editor.set_cur_default();
    }
}
