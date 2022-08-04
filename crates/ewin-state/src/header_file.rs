use ewin_key::{files::file::File, model::*};

impl HeaderFile {
    pub fn new(filenm_str: &str) -> Self {
        let file = File::new(filenm_str);
        HeaderFile { file, ..HeaderFile::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderFile {
    pub filenm_disp: String,
    pub file: File,
    pub is_disp: bool,
    pub filenm_area: (usize, usize),
    pub close_area: (usize, usize),
    pub watch_mode: WatchMode,
}

impl Default for HeaderFile {
    fn default() -> Self {
        HeaderFile { filenm_disp: String::new(), file: File::default(), is_disp: false, filenm_area: (0, 0), close_area: (0, 0), watch_mode: WatchMode::default() }
    }
}
