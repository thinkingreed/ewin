use ewin_const::{def::*, term::*};

impl View {
    pub fn is_range(&self, y: usize, x: usize) -> bool {
        return self.y <= y && y < self.y + self.height && self.x <= x && x < self.x + self.width;
    }
    pub fn is_range_around(&self, y: usize, x: usize) -> bool {
        let sy = if y == 0 { 0 } else { self.y - 1 };
        let ey = if get_term_size().1 == y { y } else { self.y + 1 };
        let sx = if x == 0 { 0 } else { self.x - 1 };
        let ex = if get_term_size().0 == x { x } else { self.x + self.width + 1 };
        return sy <= y && y <= ey && sx <= x && x <= ex;
    }
    pub fn is_x_range(&self, x: usize) -> bool {
        return self.x <= x && x <= self.x + self.width;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct View {
    pub x: usize,
    pub y: usize,
    pub y_org: usize,
    pub width: usize,
    pub height: usize,
    pub is_on_mouse: bool,
}

impl Default for View {
    fn default() -> Self {
        View { x: USIZE_UNDEFINED, y: USIZE_UNDEFINED, y_org: 0, width: 0, height: 0, is_on_mouse: false }
    }
}
