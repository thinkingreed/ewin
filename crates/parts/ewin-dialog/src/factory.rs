use crate::{
    btn_grourp::*,
    cont::{
        cont::*,
        parts::kvs::{about_app::*, file_prop::*},
    },
    dialog::*,
};
use ewin_cfg::lang::lang_cfg::*;
use ewin_const::term::*;
use ewin_key::util::*;

impl Dialog {
    pub const OK_BTN_STR: &'static str = "OK";

    pub fn to_dialog(cont_type: DialogContType) -> Dialog {
        let mut dialog = match cont_type {
            DialogContType::FileProp => Dialog {
                is_show: true,
                cont: Box::new(DialogContFileProp { base: DialogContBase { title: format!("{}{}", Lang::get().file, Lang::get().file_property), cfg: DialogContCfg { max_width: (get_term_size().0 as f32 * 0.7) as usize, ..DialogContCfg::default() }, ..DialogContBase::default() } }),
                btn_group: DialogBtnGrourp::create_grourp(DialogBtnGrourpType::Ok),
                ..Dialog::default()
            },
            DialogContType::AboutApp => Dialog {
                is_show: true,
                cont: Box::new(DialogContAboutApp { base: DialogContBase { title: Lang::get().about_app.to_string(), cfg: DialogContCfg { max_width: (get_term_size().0 as f32 * 0.9) as usize, ..DialogContCfg::default() }, ..DialogContBase::default() } }),
                btn_group: DialogBtnGrourp::create_grourp(DialogBtnGrourpType::Ok),

                ..Dialog::default()
            },
        };
        dialog.cont.as_mut_base().cfg.min_width = get_str_width(&dialog.cont.as_base().title) + Dialog::HEADER_MARGIN + Dialog::CLOSE_BTN_WIDTH;

        return dialog;
    }
}
