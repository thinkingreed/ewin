use clap::{App, Arg};
use std::collections::HashMap;

lazy_static! {
    pub static ref ARGS: HashMap<&'static str, String> = {
        let matches = App::new("ew")
            .about("A text editor")
            .bin_name("ew")
            .arg(Arg::with_name("file").required(true))
            .arg(Arg::with_name("-debug").help("debug mode").short("-d").long("-debug"))
            .get_matches();

        let file_path: String = matches.value_of_os("file").unwrap().to_string_lossy().to_string();

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
