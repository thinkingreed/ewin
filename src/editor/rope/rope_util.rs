extern crate ropey;

pub fn is_line_end(c: char) -> bool {
    ['\u{000a}', '\u{000d}'].contains(&c)
}
