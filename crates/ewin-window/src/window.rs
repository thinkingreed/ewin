use crossterm::{cursor::Hide, execute};
use std::io::Write;

pub trait Window {
    fn init(&mut self);
    fn set_disp_name(&mut self);
    fn get_draw_range_y(&mut self, offset_y: usize, hbar_disp_row_num: usize, editor_row_len: usize) -> Option<(usize, usize)>;
    fn clear(&mut self);
    fn draw(&mut self, str_vec: &mut Vec<String>);
    fn draw_only<T: Write>(&mut self, out: &mut T) {
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        execute!(out, Hide).unwrap();
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
