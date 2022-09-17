impl FileBarState {
    pub fn clear(&mut self) {
        self.is_dragging = false;
    }
}
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct FileBarState {
    pub is_dragging: bool,
}
