use crate::dialog::*;
use ewin_key::util::*;

impl DialogBtnGrourp {
    pub fn create_grourp(btn_group_type: DialogBtnGrourpType) -> DialogBtnGrourp {
        return match btn_group_type {
            DialogBtnGrourpType::Ok => DialogBtnGrourp { btn_type: DialogBtnGrourpType::Ok, vec: vec![DialogBtn { name: Dialog::OK_BTN_STR.to_string(), name_width: get_str_width(&Dialog::OK_BTN_STR), btn_type: DialogBtnType::Ok, cfg: DialogBtnCfg { is_close: true }, ..DialogBtn::default() }] },
            DialogBtnGrourpType::OkCancel => DialogBtnGrourp::default(),
        };
    }
}

#[derive(Debug, PartialEq, Default, Eq, Clone, Hash, Copy)]
pub enum DialogBtnGrourpType {
    #[default]
    Ok,
    OkCancel,
}

#[derive(Debug, Clone, Default)]
pub struct DialogBtnGrourp {
    pub btn_type: DialogBtnGrourpType,
    pub vec: Vec<DialogBtn>,
}
