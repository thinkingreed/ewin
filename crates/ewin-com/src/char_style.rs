use crate::{
    _cfg::model::default::*,
    colors::{Color, Rgb},
    model::Cell,
};

use crossterm::style::{Color as CrosstermColor, SetBackgroundColor, SetForegroundColor};
use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CharStyle {
    pub fg: Color,
    pub bg: Color,
}

impl fmt::Display for CharStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CharStyle fg:{:?}, bg:{:?},", self.fg, self.bg,)
    }
}
impl From<syntect::highlighting::Color> for Color {
    fn from(scolor: syntect::highlighting::Color) -> Self {
        Self { rgb: Rgb { r: scolor.r, g: scolor.g, b: scolor.b } }
    }
}
impl From<Color> for CrosstermColor {
    fn from(c: Color) -> crossterm::style::Color {
        crossterm::style::Color::Rgb { r: c.rgb.r, g: c.rgb.g, b: c.rgb.b }
    }
}

impl CharStyle {
    pub fn from_syntect_style(cfg: &Cfg, style: &syntect::highlighting::Style) -> CharStyle {
        if cfg.general.colors.theme.theme_bg_enable {
            CharStyle { bg: style.background.into(), fg: style.foreground.into() }
        } else {
            CharStyle { bg: cfg.colors.editor.bg, fg: style.foreground.into() }
        }
    }

    pub fn normal(cfg: &Cfg) -> CharStyle {
        let editor = &cfg.colors.editor;
        CharStyle { fg: Color { rgb: Rgb { r: editor.fg.rgb.r, g: editor.fg.rgb.g, b: editor.fg.rgb.b } }, bg: Color { rgb: Rgb { r: editor.bg.rgb.r, g: editor.bg.rgb.g, b: editor.bg.rgb.b } } }
    }

    pub fn none() -> CharStyle {
        // The set value has no meaning and is used as the initial value.
        CharStyle { fg: Color { rgb: Rgb { r: 99, g: 99, b: 99 } }, bg: Color { rgb: Rgb { r: 99, g: 99, b: 99 } } }
    }

    pub fn control_char(cfg: &Cfg) -> CharStyle {
        let control_char = &cfg.colors.editor.control_char;
        let editor = &cfg.colors.editor;
        CharStyle { fg: Color { rgb: Rgb { r: control_char.fg.rgb.r, g: control_char.fg.rgb.g, b: control_char.fg.rgb.b } }, bg: Color { rgb: Rgb { r: editor.bg.rgb.r, g: editor.bg.rgb.g, b: editor.bg.rgb.b } } }
    }
    pub fn column_char_width_gap_space(cfg: &Cfg) -> CharStyle {
        let fg = &cfg.colors.editor.column_char_width_gap_space.fg;
        let bg = &cfg.colors.editor.column_char_width_gap_space.bg;
        CharStyle { fg: Color { rgb: Rgb { r: fg.rgb.r, g: fg.rgb.g, b: fg.rgb.b } }, bg: Color { rgb: Rgb { r: bg.rgb.r, g: bg.rgb.g, b: bg.rgb.b } } }
    }

    pub fn selected(cfg: &Cfg) -> CharStyle {
        let selection = &cfg.colors.editor.selection;
        CharStyle { fg: Color { rgb: Rgb { r: selection.fg.rgb.r, g: selection.fg.rgb.g, b: selection.fg.rgb.b } }, bg: Color { rgb: Rgb { r: selection.bg.rgb.r, g: selection.bg.rgb.g, b: selection.bg.rgb.b } } }
    }

    pub fn searched(cfg: &Cfg) -> CharStyle {
        let search = &cfg.colors.editor.search;
        CharStyle { fg: Color { rgb: Rgb { r: search.fg.rgb.r, g: search.fg.rgb.g, b: search.fg.rgb.b } }, bg: Color { rgb: Rgb { r: search.bg.rgb.r, g: search.bg.rgb.g, b: search.bg.rgb.b } } }
    }
}

impl Cell {
    pub fn draw_style(&self, str_vec: &mut Vec<String>, forced_change: bool) {
        if self.from.fg != self.to.fg || forced_change {
            str_vec.push(SetForegroundColor(CrosstermColor::from(self.to.fg)).to_string());
        }
        if self.from.bg != self.to.bg || forced_change {
            str_vec.push(SetBackgroundColor(CrosstermColor::from(self.to.bg)).to_string());
        }
    }
}
