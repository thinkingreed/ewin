use crate::{ewin_core::_cfg::keys::*, ewin_core::def::*, ewin_core::global::*, ewin_core::log::*, ewin_core::model::*, model::*};
use serde::Serialize;
use serde_json::Value;

impl Editor {
    pub fn format(&mut self, fmt_type: FmtType) -> Option<String> {
        if let Err(err) = self.exec_format(fmt_type) {
            let err_str = format!("{}{}", fmt_type, LANG.parsing_failed);
            Log::error(&err_str, &err);
            return Some(err_str);
        } else {
            return None;
        }
    }

    pub fn exec_format(&mut self, fmt_type: FmtType) -> anyhow::Result<()> {
        Log::debug_key(&format!("{}:{}", "Editor.format", fmt_type));

        let format_str = match fmt_type {
            FmtType::JSON => {
                let slice = self.buf.slice_rope(self.sel.get_range());

                let value: Value = serde_json::from_str(&slice.to_string()).unwrap();
                let buf = Vec::new();
                let indent = &CFG.get().unwrap().try_lock().unwrap().general.editor.format.indent;
                let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
                let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
                value.serialize(&mut ser).unwrap();
                let mut format_str = String::from_utf8(ser.into_inner()).unwrap();
                // New line code conversion
                if self.h_file.nl == NEW_LINE_CRLF_STR {
                    format_str = format_str.replace(NEW_LINE_LF, NEW_LINE_CRLF)
                }
                format_str
            }
            FmtType::XML | FmtType::HTML => {
                let slice = self.buf.slice(self.sel.get_range());
                let nl = NL::get_nl(&self.h_file.enc.to_string());
                if fmt_type == FmtType::XML {
                    FormatXml::format_xml_html(slice, FmtType::XML, nl)
                } else {
                    FormatXml::format_xml_html(slice, FmtType::HTML, nl)
                }
            }
        };

        Log::debug("format_str", &format_str);

        self.keycmd = KeyCmd::InsertStr(format_str.clone());
        self.edit_proc(KeyCmd::InsertStr(format_str));

        Ok(())
    }
}