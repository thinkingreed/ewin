use crate::{_cfg::keys::KeyCmd, log::*, model::*};
extern crate serde_json;
use serde_json::Value;

impl Editor {
    pub fn format(&mut self, fmt_type: FmtType) -> anyhow::Result<()> {
        Log::debug_key(&format!("{}:{}", "Editor.format", fmt_type));

        let slice = self.buf.slice_rope(self.sel.get_range());
        let format_str = match fmt_type {
            FmtType::JSON => {
                let value: Value = serde_json::from_str(&format!(r#"{}"#, slice))?;
                serde_json::to_string_pretty(&value).unwrap()
            }
            FmtType::XML => "".to_string(),
        };
        self.edit_proc(KeyCmd::InsertStr(format_str));
        Ok(())
    }
}
