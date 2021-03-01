use crate::{
    global::*,
    help::{Help, HelpMode},
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
use std::{
    ffi::OsStr,
    io::{self, stdout, Write},
    path::Path,
    path::PathBuf,
    process,
    process::Command,
};
#[derive(Debug)]
pub struct Terminal {}

pub struct Args {
    pub mode: ActivatMode,
    pub filenm: String,
    pub ext: String,
    pub search_str: String,
    // full path
    pub search_file: String,
    pub search_folder: String,
    pub search_filenm: String,
    pub search_case_sens: bool,
    pub search_regex: bool,
    pub search_row_num: String,
}
impl Default for Args {
    fn default() -> Self {
        Args {
            mode: ActivatMode::Nomal,
            filenm: String::new(),
            ext: String::new(),
            search_str: String::new(),
            search_file: String::new(),
            search_folder: String::new(),
            search_filenm: String::new(),
            search_case_sens: true,
            search_regex: false,
            search_row_num: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivatMode {
    Nomal,
    Grep,
    GrepResult,
}

impl Terminal {
    pub fn draw<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("　　　　　　　　All draw");

        Terminal::set_disp_size(editor, mbar, prom, help, sbar);

        let d_range = editor.d_range;
        Log::ep("d_range", &d_range);

        if !(d_range.draw_type == DrawType::Not || d_range.draw_type == DrawType::MoveCur) {
            editor.draw_cache();
            editor.draw(out);
        }

        let mut str_vec: Vec<String> = vec![];

        help.draw(&mut str_vec);
        mbar.draw(&mut str_vec);
        prom.draw(&mut str_vec);
        if d_range.draw_type != DrawType::Not {
            sbar.draw(&mut str_vec, editor);
        }
        Terminal::draw_cur(&mut str_vec, editor, prom);

        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush()?;

        return Ok(());
    }

    pub fn draw_cur(str_vec: &mut Vec<String>, editor: &mut Editor, prom: &mut Prompt) {
        Log::ep_s("　　　　　　　set_cur_str");

        if prom.is_save_new_file || prom.is_search || prom.is_replace || prom.is_grep || prom.is_move_line {
            prom.draw_cur(str_vec);
        } else {
            /*
            Log::ep("cur.disp_x - 1 - editor.offset_disp_x", &(editor.cur.disp_x - 1 - editor.offset_disp_x));
            Log::ep("cur.disp_x ", &editor.cur.disp_x);
            Log::ep("offset_disp_x", &editor.offset_disp_x);
            Log::ep("cur.y", &editor.cur.y);
            Log::ep("offset_y", &editor.offset_y);
            */
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
    pub fn startup_terminal(args: String) {
        Log::ep("args", &args);

        let exe_path = if !cfg!(debug_assertions) && Path::new("/usr/bin/ewin").exists() { "/usr/bin/ewin" } else { "/home/hi/rust/ewin/target/release/ewin" };

        if *ENV == Env::WSL {
            if let Err(err) = Command::new("/mnt/c/windows/system32/cmd.exe")
                .arg("/c")
                .arg("start")
                .arg("wsl")
                .arg("-e")
                .arg(exe_path)
                .arg(args)
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .spawn()
            {
                Log::ep_s("WSL");
                Log::ep("startup_terminal err", &err.to_string());
            }
        } else {
            // gnome-terminal
            if let Err(err) = Command::new("gnome-terminal").arg("--").arg(exe_path).arg(args).spawn() {
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
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, ResetColor, Show).unwrap();
    }

    pub fn init_args(file_path: &String) -> Args {
        let activate_mode = Terminal::check_activate_mode(&file_path);
        let mut args = Args::default();
        args.mode = activate_mode;

        if activate_mode == ActivatMode::GrepResult || activate_mode == ActivatMode::Grep {
            let v: Vec<&str> = file_path.split_ascii_whitespace().collect();
            args.search_str = v[0].split("=").nth(1).unwrap().to_string();
            args.search_file = v[1].split("=").nth(1).unwrap().to_string();
            args.search_case_sens = if v[2].split("=").nth(1).unwrap() == "true" { true } else { false };
            args.search_regex = if v[3].split("=").nth(1).unwrap() == "true" { true } else { false };

            let path = PathBuf::from(&args.search_file);
            let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
            let path_str = path.to_string_lossy().to_string();
            args.search_folder = path_str.replace(&filenm, "");
            args.ext = Path::new(&filenm).extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
            args.search_filenm = filenm;

            if activate_mode == ActivatMode::GrepResult {
                let search_row_nums: Vec<&str> = v[4].split("=").collect();
                args.search_row_num = search_row_nums[1].to_string();
            }
        // Normal
        } else {
            if !file_path.is_empty() {
                args.filenm = file_path.to_string();
                args.ext = Path::new(&args.filenm).extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
            }
        }
        return args;
    }

    pub fn activate(args: &Args, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) {
        // grep_result

        editor.file.ext = args.ext.clone();

        if args.mode == ActivatMode::GrepResult || args.mode == ActivatMode::Grep {
            editor.search.str = args.search_str.clone();
            editor.search.file = args.search_file.clone();
            CFG.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.case_sens = args.search_case_sens).unwrap();
            CFG.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.regex = args.search_regex).unwrap();
            editor.search.folder = args.search_folder.clone();
            editor.search.filenm = args.search_filenm.clone();

            if args.mode == ActivatMode::Grep {
                sbar.filenm = format!("grep \"{}\" {}", &editor.search.str, &editor.search.file);
                prom.is_grep_result = true;
                prom.is_grep_stdout = true;
                prom.is_grep_stderr = true;

                prom.grep_result(editor, mbar, help, sbar);
                editor.set_cur_default();
                editor.scroll();
                editor.scroll_horizontal();
                mbar.set_info(&LANG.searching);
            } else {
                sbar.filenm = args.search_file.clone();
                editor.search.row_num = args.search_row_num.clone();

                Log::ep("editor.search", &editor.search);

                editor.open(Path::new(&sbar.filenm), mbar);
                editor.search_str(true, false);
            }

        // Normal
        } else {
            sbar.filenm = args.filenm.clone();
            editor.open(Path::new(&args.filenm), mbar);
        }
    }

    pub fn check_activate_mode(file_path: &String) -> ActivatMode {
        if file_path.match_indices("search_str").count() > 0 {
            if file_path.match_indices("search_row_num").count() == 0 {
                return ActivatMode::Grep;
            } else {
                return ActivatMode::GrepResult;
            }
        } else {
            return ActivatMode::Nomal;
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
