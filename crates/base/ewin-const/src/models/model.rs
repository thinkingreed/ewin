#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    Nomal,
    Delim,
    HalfSpace,
    FullSpace,
    NewLineCode,
}

// Cursor direction
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum InputCompleMode {
    None,
    WordComple,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WindowSplitType {
    #[default]
    None,
    Vertical,
    Horizontal,
}

// Keys without modifiers
#[derive(Debug, Copy, Default, Clone, Hash, Eq, PartialEq)]
pub enum OpenFileType {
    #[default]
    Normal,
    JsMacro,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum WatchMode {
    // Warning every time it is changed by another app
    #[default]
    Normal,
    NotMonitor,
    NotEditedWillReloadedAuto,
}
