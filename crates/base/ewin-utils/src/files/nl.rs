use ewin_const::def::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

impl NL {
    pub fn check_nl(file: &File) -> String {
        let mut new_line = NEW_LINE_LF_STR.to_string();
        let reader = BufReader::new(file);
        for (idx, line_result) in reader.lines().enumerate() {
            // Judge by a small number of lines, not the whole line
            if idx > 5 {
                break;
            }
            if let Ok(line) = line_result {
                if line.contains(&new_line) {
                    new_line = NEW_LINE_CRLF_STR.to_string();
                    break;
                }
            }
        }
        return new_line;
    }

    pub fn get_nl(nl_str: &str) -> String {
        if nl_str == NEW_LINE_CRLF_STR {
            NEW_LINE_CRLF.to_string()
        } else {
            NEW_LINE_LF.to_string()
        }
    }

    pub fn change_nl(string: &mut String, to_nl: &str) {
        // Since it is not possible to replace only LF from a character string containing CRLF,
        // convert it to LF and then convert it to CRLF.
        *string = string.replace(NEW_LINE_CRLF, &NEW_LINE_LF.to_string());
        if to_nl == NEW_LINE_CRLF_STR {
            *string = string.replace(&NEW_LINE_LF.to_string(), NEW_LINE_CRLF);
        }
    }

    pub fn del_nl(string: &mut String) {
        *string = string.replace(NEW_LINE_CRLF, "");
        *string = string.replace(NEW_LINE_LF, "");
    }
}

pub struct NL {}
