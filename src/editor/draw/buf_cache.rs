extern crate ropey;
use crate::{editor::draw::char_style::*, model::*};
use rayon::prelude::*;
use std::cmp::min;
use syntect::easy::HighlightFile;
use syntect::easy::HighlightLines;

impl Editor {
    pub fn draw_cache(&mut self) {
        Log::ep_s("　　　　　　　draw_cache");

        // char_vec initialize
        let diff: isize = self.buf.len_lines() as isize - self.draw.char_vec.len() as isize;
        if diff > 0 {
            self.draw.char_vec.resize_with(self.buf.len_lines() as usize, || vec![]);
            self.draw.regions.resize_with(self.buf.len_lines() as usize, || vec![]);
        }

        self.draw.sy = self.offset_y;
        self.draw.ey = min(self.buf.len_lines() - 1, self.offset_y + self.disp_row_num);

        let d_range = self.d_range.get_range();
        if d_range.d_type == DrawType::Target {
            self.draw.sy = d_range.sy;
            self.draw.ey = d_range.ey;
        } else if d_range.d_type == DrawType::After {
            self.draw.sy = d_range.sy;
        }

        if self.history.len_history() > 0 {
            let hist: &HistoryInfo = self.history.get_history_last();
            let ep = hist.evt_proc.clone();
            match ep.d_range.d_type {
                DrawType::Target | DrawType::After | DrawType::All | DrawType::None => {
                    if self.is_edit_evt(true) {
                        Log::ep_s("refresh refresh refresh refresh refresh");
                        for i in self.draw.sy..=self.draw.ey {
                            self.draw.char_vec[i] = self.buf.char_vec_line(i);
                        }
                    }
                }
                DrawType::Not => {}
            }
        }

        Log::ep("self.draw.sy", self.draw.sy);
        Log::ep("self.draw.ey", self.draw.ey);
        // Initial display line
        let mut h = HighlightLines::new(&self.syntax.syntax, &self.syntax.theme_set.themes["base16-ocean.dark"]);

        let sel_ranges = self.sel.get_range();
        let search_ranges = self.search.ranges.clone();

        for y in self.draw.sy..=self.draw.ey {
            if self.draw.char_vec[y].len() == 0 {
                let mut regions: Vec<Region> = vec![];
                let row_vec = self.buf.char_vec_line(y);
                let row = row_vec.par_iter().collect::<String>();
                let mut i = 0;
                for (style, string) in h.highlight(&row, &self.syntax.syntax_set) {
                    Log::ep("style", CharStyle::from(style));
                    Log::ep("string", string);
                    let mut style_type_org = CharStyleType::None;
                    for (xx, c) in string.chars().enumerate() {
                        Log::ep("i", i);
                        //      Log::ep("x", x);
                        //      Log::ep("xx", xx);
                        //      Log::ep("x + xx", x + xx);
                        //      Log::ep("ccc", c);

                        let style_type = self.ctrl_charstyletype(&row_vec, &sel_ranges, &search_ranges, y, i);
                        let mut char_style = None;
                        match style_type {
                            CharStyleType::Select => {
                                if style_type != style_type_org {
                                    char_style = Some(styles::SELECTED);
                                }
                                regions.push(Region { c, to: char_style, from: styles::DEFAULT });
                            }
                            CharStyleType::None => {
                                if xx == 0 {
                                    char_style = Some(CharStyle::from(style));
                                }
                                regions.push(Region { c, to: char_style, from: styles::DEFAULT });
                            }
                            CharStyleType::CtrlChar => regions.push(Region { c, to: Some(styles::CTRL_CHAR), from: styles::DEFAULT }),
                        }
                        style_type_org = style_type;
                        i += 1;
                    }
                }
                self.draw.char_vec[y] = row_vec;
                self.draw.regions[y] = regions;
                //     eprintln!("self.draw.regions[y] {:?}", self.draw.regions[y]);
            }
        }

        for i in self.draw.sy..=self.draw.ey {
            if self.draw.char_vec[i].len() == 0 {
                self.draw.char_vec[i] = self.buf.char_vec_line(i);
            }
        }
        //  eprintln!("self.draw.char_vec {:?}", self.draw.char_vec);
        //  eprintln!("self.draw.regions {:?}", self.draw.regions);
    }
}
