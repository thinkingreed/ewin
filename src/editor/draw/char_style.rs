use crate::model::*;
use std;
use std::collections::BTreeMap;
use std::fmt;
use syntect;
use termion;
use termion::color::{Bg, Fg};

impl Into<Box<dyn termion::color::Color>> for Color {
    fn into(self) -> Box<dyn termion::color::Color> {
        match self {
            Color::Reset => Box::new(termion::color::Reset),
            Color::Rgb { r, g, b } => Box::new(termion::color::Rgb(r, g, b)),
        }
    }
}

impl Color {
    fn to_ansi(self) -> Box<dyn termion::color::Color> {
        match self {
            Color::Reset => Box::new(termion::color::Reset),
            Color::Rgb { r, g, b } => Box::new(termion::color::AnsiValue(ansi_colours::ansi256_from_rgb((r, g, b)))),
        }
    }
}

impl From<syntect::highlighting::Color> for Color {
    fn from(scolor: syntect::highlighting::Color) -> Self {
        Self::Rgb { r: scolor.r, g: scolor.g, b: scolor.b }
    }
}

impl CharStyle {
    pub fn fg(fg: Color) -> Self {
        Self { fg, bg: Default::default() }
    }
    pub fn bg(bg: Color) -> Self {
        Self { fg: Default::default(), bg }
    }
    pub fn fg_bg(fg: Color, bg: Color) -> Self {
        Self { fg, bg }
    }

    pub const DEFAULT_BG: Color = Color::Rgb { r: 0, g: 0, b: 0 };

    pub const NONE: CharStyle = CharStyle {
        fg: Color::Rgb { r: 99, g: 99, b: 99 },
        bg: Color::Rgb { r: 99, g: 99, b: 99 },
    };
    pub const DEFAULT: CharStyle = CharStyle {
        fg: Color::Rgb { r: 255, g: 255, b: 255 },
        bg: Color::Rgb { r: 0, g: 0, b: 0 },
    };
    pub const HIGHLIGHT: CharStyle = CharStyle {
        fg: Color::Rgb { r: 255, g: 0, b: 0 },
        bg: Color::Rgb { r: 0, g: 0, b: 0 },
    };
    pub const CTRL_CHAR: CharStyle = CharStyle {
        fg: Color::Rgb { r: 110, g: 110, b: 110 },
        bg: Color::Rgb { r: 0, g: 0, b: 0 },
    };

    pub const SELECTED: CharStyle = CharStyle {
        fg: Color::Rgb { r: 0, g: 0, b: 0 },
        bg: Color::Rgb { r: 221, g: 72, b: 20 },
    };
}
pub struct StyleWithColorType {
    pub is_ansi_color: bool,
    pub style: CharStyle,
}

impl fmt::Display for StyleWithColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_ansi_color {
            write!(f, "{}{}", Fg(self.style.fg.to_ansi().as_ref()), Bg(self.style.bg.to_ansi().as_ref()))
        } else {
            write!(f, "{}{}", Fg(Into::<Box<dyn termion::color::Color>>::into(self.style.fg).as_ref()), Bg(Into::<Box<dyn termion::color::Color>>::into(self.style.bg).as_ref()),)
        }
    }
}

impl Region {
    pub fn draw_style(&self, str_vec: &mut Vec<String>, forced_change: bool) {
        // TODO ansi_color
        if self.from.fg != self.to.fg || forced_change {
            str_vec.push(Fg(Into::<Box<dyn termion::color::Color>>::into(self.to.fg).as_ref()).to_string());
        }
        if self.from.bg != self.to.bg || forced_change {
            str_vec.push(Bg(Into::<Box<dyn termion::color::Color>>::into(self.to.bg).as_ref()).to_string());
        }
    }
}
