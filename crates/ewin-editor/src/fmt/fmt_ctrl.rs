use crate::{ewin_com::model::*, model::*};
use ewin_cfg::{
    lang::lang_cfg::*,
    log::*,
    model::{default::*, modal::*},
};
use ewin_com::_cfg::key::cmd::{Cmd, CmdType};
use ewin_const::def::*;
use serde::Serialize;
use serde_json::Value;

impl Editor {
    pub fn format(&mut self, fmt_type: FileType) -> Option<String> {
        if !self.sel.is_selected() {
            Some(Lang::get().no_sel_range.to_string())
        } else if let Err(err) = self.exec_format(fmt_type) {
            let err_str = format!("{}{}", fmt_type, Lang::get().parsing_failed);
            Log::error(&err_str, &err);
            Some(err_str)
        } else {
            None
        }
    }

    pub fn exec_format(&mut self, fmt_type: FileType) -> anyhow::Result<()> {
        Log::debug_key(&format!("{}:{}", "Editor.format", fmt_type));

        let format_str = match fmt_type {
            FileType::JSON => {
                let value: Value = serde_json::from_str(&self.buf.slice_string(self.sel.get_range()))?;
                let buf = Vec::new();
                let indent = &Cfg::get().general.editor.format.indent;
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
            FileType::XML | FileType::HTML => {
                let slice = self.buf.slice(self.sel.get_range());
                let nl = NL::get_nl(&self.h_file.nl.to_string());
                FormatXml::format_xml_html(slice, fmt_type, nl)
            }
            _ => todo!(),
        };

        self.cmd = Cmd::to_cmd(CmdType::InsertStr(format_str));
        self.edit_proc(self.cmd.clone());

        Ok(())
    }
}
