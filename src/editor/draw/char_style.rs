use crate::model::*;
use crossterm::style::{Color as CrosstermColor, SetBackgroundColor, SetForegroundColor};
use syntect;

/*
impl Into<Box<dyn termion::color::Color>> for Color {
    fn into(self) -> Box<dyn termion::color::Color> {
        match self {
            Color::Rgb { r, g, b } => Box::new(termion::color::Rgb(r, g, b)),
        }
    }
}
impl Color {
    fn to_ansi(self) -> Box<dyn termion::color::Color> {
        match self {
            Color::Rgb { r, g, b } => Box::new(termion::color::AnsiValue(ansi_colours::ansi256_from_rgb((r, g, b)))),
        }
    }
}*/

impl From<syntect::highlighting::Color> for Color {
    fn from(scolor: syntect::highlighting::Color) -> Self {
        Self { r: scolor.r, g: scolor.g, b: scolor.b }
    }
}

/*
impl Into<Color> for crossterm::style::Color {
    fn into(self) -> crossterm::style::Color {
        crossterm::style::Color {  self.into() };
    }
}*/

impl From<Color> for crossterm::style::Color {
    fn from(c: Color) -> crossterm::style::Color {
        crossterm::style::Color::Rgb { r: c.r, g: c.g, b: c.b }
    }
}

impl CharStyle {
    pub const DEFAULT_BG: Color = Color { r: 0, g: 0, b: 0 };

    pub const NONE: CharStyle = CharStyle {
        fg: Color { r: 99, g: 99, b: 99 },
        bg: Color { r: 99, g: 99, b: 99 },
    };
    pub const DEFAULT: CharStyle = CharStyle {
        fg: Color { r: 255, g: 255, b: 255 },
        bg: Color { r: 0, g: 0, b: 0 },
    };
    pub const HIGHLIGHT: CharStyle = CharStyle {
        fg: Color { r: 255, g: 0, b: 0 },
        bg: Color { r: 0, g: 0, b: 0 },
    };
    pub const CTRL_CHAR: CharStyle = CharStyle {
        fg: Color { r: 110, g: 110, b: 110 },
        bg: Color { r: 0, g: 0, b: 0 },
    };

    pub const SELECTED: CharStyle = CharStyle {
        fg: Color { r: 0, g: 0, b: 0 },
        bg: Color { r: 221, g: 72, b: 20 },
    };
}
/*
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
}*/

impl Region {
    pub fn draw_style(&self, str_vec: &mut Vec<String>, forced_change: bool) {
        // TODO ansi_color
        if self.from.fg != self.to.fg || forced_change {
            str_vec.push(SetForegroundColor(CrosstermColor::from(self.to.fg)).to_string());
        }
        if self.from.bg != self.to.bg || forced_change {
            str_vec.push(SetBackgroundColor(CrosstermColor::from(self.to.bg)).to_string());
        }
    }
}
