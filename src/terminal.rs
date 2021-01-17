use crate::{_cfg::lang::lang_cfg::*, global::ENV, model::*};
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::process::Command;
use termion::cursor;

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("　　　　　　　　All draw");

        self.set_disp_size(editor, mbar, prom, sbar);

        let str_vec: &mut Vec<String> = &mut vec![];

        // Redraw in case of msg change
        if mbar.msg_org != mbar.msg {
            editor.d_range.d_type = DrawType::All;
        }
        let d_range = editor.d_range;
        Log::ep("d_range", d_range);

        if d_range.d_type != DrawType::Not {
            Log::ep_s("editor.draw");
            editor.draw_cache();
            editor.draw(str_vec);
        }

        mbar.draw(str_vec);
        prom.draw(str_vec);

        if d_range.d_type != DrawType::Not {
            sbar.draw(str_vec, editor);
        }
        self.draw_cur(str_vec, editor, prom);

        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush()?;

        return Ok(());
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor, prom: &mut Prompt) {
        // Log::ep_s("　　　　　　　set_cur_str");

        if prom.is_save_new_file || prom.is_search || prom.is_replace || prom.is_grep {
            prom.draw_cur(str_vec);
        } else {
            str_vec.push(cursor::Goto((editor.cur.disp_x - editor.offset_disp_x) as u16, (editor.cur.y + 1 - editor.offset_y) as u16).to_string());
        }
    }

    pub fn check_displayable(&mut self, lang_cfg: &LangCfg) -> bool {
        let (cols, rows) = termion::terminal_size().unwrap();
        if cols < 20 || rows < 8 {
            println!("{:?}", lang_cfg.terminal_size_small);
            return false;
        }
        return true;
    }

    pub fn set_disp_size(&mut self, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) {
        let (cols, rows) = termion::terminal_size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);

        Log::ep("rows", rows);
        Log::ep("cols", cols);

        if rows <= 10 {
            sbar.disp_row_num = 0;
        } else {
            sbar.disp_row_num = 1;
            sbar.disp_row_posi = rows;
            sbar.disp_col_num = cols;
        }
        prom.disp_row_posi = rows - prom.disp_row_num + 1 - sbar.disp_row_num;

        mbar.disp_col_num = cols;
        if mbar.msg_readonly.len() > 0 {
            mbar.disp_readonly_row_num = 1;
        } else {
            mbar.disp_readonly_row_num = 0;
        }
        if mbar.msg_keyrecord.len() > 0 {
            mbar.disp_keyrecord_row_num = 1;
        } else {
            mbar.disp_keyrecord_row_num = 0;
        }
        if mbar.msg.len() > 0 {
            mbar.disp_row_num = 1;
        } else {
            mbar.disp_row_num = 0;
        }

        mbar.disp_row_posi = rows - prom.disp_row_num - sbar.disp_row_num;
        mbar.disp_keyrecord_row_posi = rows - mbar.disp_row_num - prom.disp_row_num - sbar.disp_row_num;
        mbar.disp_readonly_row_posi = rows - mbar.disp_keyrecord_row_num - mbar.disp_row_num - prom.disp_row_num - sbar.disp_row_num;

        editor.disp_col_num = cols;
        editor.disp_row_num = rows - mbar.disp_readonly_row_num - mbar.disp_keyrecord_row_num - mbar.disp_row_num - prom.disp_row_num - sbar.disp_row_num;

        /*
            Log::ep("editor.disp_row_num", editor.disp_row_num);
            Log::ep("mbar.disp_keyrecord_row_posi", mbar.disp_keyrecord_row_posi);
            Log::ep("mbar.disp_row_posi", mbar.disp_row_posi);
            Log::ep("prom.disp_row_posi", prom.disp_row_posi);
            Log::ep("sbar.disp_row_posi", sbar.disp_row_posi);
        */
    }

    pub fn show_cur<T: Write>(&mut self, out: &mut T) {
        write!(out, "{}", cursor::Show).unwrap();
        out.flush().unwrap();
    }
    pub fn hide_cur<T: Write>(&mut self, out: &mut T) {
        write!(out, "{}", cursor::Hide.to_string()).unwrap();
        out.flush().unwrap();
    }
    pub fn startup_terminal(&mut self, search_strs: String) {
        Log::ep("search_strs", search_strs.clone());

        let mut exe_path = "/home/hi/rust/ewin/target/release/ewin";
        if !cfg!(debug_assertions) {
            if Path::new("/usr/bin/ewin").exists() {
                exe_path = "/usr/bin/ewin";
            } else {
                exe_path = "/home/hi/rust/ewin/target/release/ewin";
            }
        }

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
                Log::ep("startup_terminal err", err.to_string());
            }
        } else {
            // gnome-terminal
            if let Err(err) = Command::new("gnome-terminal").arg("--").arg(exe_path).arg(search_strs).spawn() {
                Log::ep_s("gnome");
                Log::ep("startup_terminal err", err.to_string());
            }
        };
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
