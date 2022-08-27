impl FilebarFile {
    pub fn new() -> Self {
        FilebarFile { ..FilebarFile::default() }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FilebarFile {
    pub filenm_disp: String,
    pub is_disp: bool,
    pub filenm_area: (usize, usize),
    pub close_area: (usize, usize),
}
