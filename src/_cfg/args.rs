use crate::model::Log;
use clap::{App, Arg};
use std::collections::HashMap;
use std::ffi::OsStr;

lazy_static! {
    pub static ref ARGS: HashMap<&'static str, String> = {
        let matches = App::new("ew")
            .about("A text editor")
            .bin_name("ew")
            .arg(Arg::with_name("file").required(false))
            .arg(Arg::with_name("-debug").help("debug mode").short("-d").long("-debug"))
            .get_matches();
        let file_path: String = matches.value_of_os("file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();

        Log::ep("file_path", file_path.clone());

        let mut debug_mode = false;
        if matches.is_present("-debug") {
            debug_mode = true;
        }

        let mut m = HashMap::new();
        m.insert("file_path", file_path);
        m.insert("debug_mode", debug_mode.to_string());
        return m;
    };
}
