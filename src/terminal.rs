use crate::{
    global::*,
    help::{self, Help},
    log::*,
    model::*,
    msgbar::*,
    prompt::prompt::*,
    statusbar::*,
};
use crossterm::{
    cursor::*,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::ResetColor,
    terminal::*,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use help::HelpMode;
use std::{
    ffi::OsStr,
    io::{self, stdout, Write},
    path::Path,
    path::PathBuf,
    process,
    process::Command,
};

impl Terminal {
    pub fn draw<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("　　　　　　　　All draw");

        Terminal::set_disp_size(editor, mbar, prom, help, sbar);
        let mut str_vec: Vec<String> = vec![];

        let d_range = editor.d_range;
        Log::ep("d_range", &d_range);

        if d_range.draw_type != DrawType::Not {
            editor.draw_cache();
            editor.draw(&mut str_vec);
        }

        help.draw(&mut str_vec);
        mbar.draw(&mut str_vec);
        prom.draw(&mut str_vec);
        sbar.draw(&mut str_vec, editor);

        Terminal::draw_cur(&mut str_vec, editor, prom);
        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush()?;

        return Ok(());
    }

    pub fn draw_cur(str_vec: &mut Vec<String>, editor: &mut Editor, prom: &mut Prompt) {
        // Log::ep_s("　　　　　　　set_cur_str");

        if prom.is_save_new_file || prom.is_search || prom.is_replace || prom.is_grep || prom.is_move_line {
            prom.draw_cur(str_vec);
        } else {
            str_vec.push(MoveTo((editor.cur.disp_x - 1 - editor.offset_disp_x) as u16, (editor.cur.y - editor.offset_y) as u16).to_string());
        }
    }

    pub fn check_displayable() -> bool {
        let (cols, rows) = size().unwrap();
        if cols <= 20 || rows <= 10 {
            println!("{:?}", &LANG.terminal_size_small);
            return false;
        }
        return true;
    }

    pub fn set_disp_size(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) {
        let (cols, rows) = size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);

        Log::ep("rows", &rows);
        Log::ep("cols", &cols);

        sbar.disp_row_num = 1;
        sbar.disp_row_posi = rows;
        sbar.disp_col_num = cols;

        Log::ep("help.mode", &help.mode);

        help.disp_row_num = if help.mode == HelpMode::Show { Help::DISP_ROW_NUM } else { 0 };
        help.disp_row_posi = if help.mode == HelpMode::Show { rows - sbar.disp_row_num - help.disp_row_num + 1 } else { 0 };

        prom.disp_col_num = cols;
        prom.disp_row_posi = rows - prom.disp_row_num + 1 - help.disp_row_num - sbar.disp_row_num;

        mbar.disp_col_num = cols;
        mbar.disp_readonly_row_num = if mbar.msg_readonly.is_empty() { 0 } else { 1 };
        mbar.disp_keyrecord_row_num = if mbar.msg_keyrecord.is_empty() { 0 } else { 1 };
        mbar.disp_row_num = if mbar.msg.str.is_empty() { 0 } else { 1 };

        mbar.disp_row_posi = rows - prom.disp_row_num - help.disp_row_num - sbar.disp_row_num;
        mbar.disp_keyrecord_row_posi = rows - mbar.disp_row_num - prom.disp_row_num - help.disp_row_num - sbar.disp_row_num;
        mbar.disp_readonly_row_posi = rows - mbar.disp_keyrecord_row_num - mbar.disp_row_num - prom.disp_row_num - help.disp_row_num - sbar.disp_row_num;

        editor.disp_col_num = cols;
        editor.disp_row_num = rows - mbar.disp_readonly_row_num - mbar.disp_keyrecord_row_num - mbar.disp_row_num - prom.disp_row_num - help.disp_row_num - sbar.disp_row_num;

        /*
           Log::ep("editor.disp_row_num", &editor.disp_row_num);
           Log::ep("mbar.disp_keyrecord_row_num", &mbar.disp_keyrecord_row_num);
           Log::ep("mbar.disp_readonly_row_num", &mbar.disp_readonly_row_num);
           Log::ep("mbar.disp_row_num", &mbar.disp_row_num);
           Log::ep("prom.disp_row_num", &prom.disp_row_num);
           Log::ep("help.disp_row_num", &help.disp_row_num);
           Log::ep("help.disp_row_posi", &help.disp_row_posi);
           Log::ep("sbar.disp_row_num", &sbar.disp_row_num);
        */
    }

    pub fn init_draw<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) {
        prom.clear();
        mbar.clear();
        editor.d_range.draw_type = DrawType::All;
        Terminal::draw(out, editor, mbar, prom, help, sbar).unwrap();
    }

    pub fn show_cur() {
        execute!(stdout(), Show).unwrap();
    }
    pub fn hide_cur() {
        execute!(stdout(), Hide).unwrap();
    }
    pub fn startup_terminal(search_strs: String) {
        Log::ep("search_strs", &search_strs);

        let exe_path = if !cfg!(debug_assertions) && Path::new("/usr/bin/ewin").exists() { "/usr/bin/ewin" } else { "/home/hi/rust/ewin/target/release/ewin" };

        if *ENV == Env::WSL {
            if let Err(err) = Command::new("/mnt/c/windows/system32/cmd.exe")
                .arg("/c")
                .arg("start")
                .arg("wsl")
                .arg("-e")
                .arg(exe_path)
                .arg(search_strs)
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .spawn()
            {
                Log::ep_s("WSL");
                Log::ep("startup_terminal err", &err.to_string());
            }
        } else {
            // gnome-terminal
            if let Err(err) = Command::new("gnome-terminal").arg("--").arg(exe_path).arg(search_strs).spawn() {
                Log::ep_s("gnome");
                Log::ep("startup_terminal err", &err.to_string());
            }
        };
    }
    pub fn init() {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    }
    pub fn exit() {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, ResetColor).unwrap();
    }

    pub fn activate(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, file_path: String) {
        // grep_result
        if file_path.match_indices("search_str").count() > 0 && file_path.match_indices("search_file").count() > 0 {
            let v: Vec<&str> = file_path.split_ascii_whitespace().collect();
            let search_strs: Vec<&str> = v[0].split("=").collect();
            editor.search.str = search_strs[1].to_string();
            let search_files: Vec<&str> = v[1].split("=").collect();
            editor.search.file = search_files[1].to_string();

            let path = PathBuf::from(&editor.search.file);
            let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
            let path_str = path.to_string_lossy().to_string();
            editor.search.folder = path_str.replace(&filenm, "");
            editor.search.filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();

            if file_path.match_indices("search_row_num").count() == 0 {
                sbar.filenm = format!("grep \"{}\" {}", &editor.search.str, &editor.search.file);
                prom.is_grep_result = true;
                prom.is_grep_stdout = true;
                prom.is_grep_stderr = true;
                prom.grep_result();
                editor.set_cur_default();
                editor.scroll();
                editor.scroll_horizontal();
                mbar.set_info(&LANG.searching);
            } else {
                sbar.filenm = editor.search.file.clone();
                let search_row_nums: Vec<&str> = v[2].split("=").collect();
                editor.search.row_num = search_row_nums[1].to_string();
                Log::ep("search_row_num", &editor.search.row_num.clone());
                editor.open(Path::new(&sbar.filenm), mbar);
                editor.search_str(true, false);
            }
        // Normal
        } else {
            if !file_path.is_empty() {
                sbar.filenm = file_path.to_string();
            }
            editor.open(Path::new(&file_path), mbar);
        }
    }
}
impl UT {
    /*
    pub fn init_ut() -> (Editor, MsgBar) {
        let mut e = Editor::default();
        e.buf = vec![vec![]];
        e.buf[0] = vec![EOF_MARK];
        e.disp_row_num = 5;
        e.set_cur_default();
        e.d_range = DRnage::default();

        let mbar = MsgBar::new();

        return (e, mbar);
    }

    pub fn insert_str(e: &mut Editor, str: &str) {
        for c in str.chars() {
            e.insert_char(c);
        }
    }
    pub fn undo_all(e: &mut Editor, mbar: &mut MsgBar) {
        let vec = e.undo_vec.clone();
        for evt_proc in vec.iter().rev() {
            Log::ep("undo_all.evt_proc.do_type", evt_proc.do_type);
            e.undo(mbar);
        }
    }
    pub fn get_buf_str(e: &mut Editor) -> String {
        let mut s = String::new();
        for vec in &e.buf {
            s.push_str(&vec.iter().collect::<String>());
        }
        return s;
    }
    */
}
