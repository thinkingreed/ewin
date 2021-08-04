use super::js_macro::MacrosFunc;
use crate::{_cfg::keys::*, global::*, log::*, model::*, sel_range::SelMode};
use rusty_v8::{self as v8, FunctionCallbackArguments, HandleScope, ReturnValue};
use v8::{Context, Local};

impl Macros {
    pub fn regist_js_func(scope: &mut v8::ContextScope<v8::HandleScope>, context: Local<Context>) {
        Macros::set_data_property(scope, context, MacrosFunc::InsText.to_string(), Macros::InsText);
        Macros::set_data_property(scope, context, MacrosFunc::GetSelectedString.to_string(), Macros::GetSelectedString);
    }

    #[allow(non_snake_case)]
    pub fn InsText(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
        // If Insert Text is not empty
        if args.get(0) != v8::undefined(scope) {
            let input_str = args.get(0).to_string(scope).unwrap();

            if let Some(Ok(mut tab)) = TAB.get().map(|tab| tab.try_lock()) {
                tab.editor.edit_proc(KeyCmd::InsertStr(input_str.to_rust_string_lossy(scope)));
                Log::macros(MacrosFunc::InsText, &input_str.to_rust_string_lossy(scope));
            }
        }
    }
    #[allow(non_snake_case)]
    pub fn GetSelectedString(scope: &mut HandleScope, _: FunctionCallbackArguments, mut rv: ReturnValue) {
        if let Some(Ok(mut tab)) = TAB.get().map(|tab| tab.try_lock()) {
            let mut sel_str = String::new();

            Log::debug("tab.editor.sel", &tab.editor.sel);

            if tab.editor.sel.is_selected() {
                sel_str = match tab.editor.sel.mode {
                    SelMode::Normal => tab.editor.buf.slice(tab.editor.sel.get_range()),
                    SelMode::BoxSelect => tab.editor.slice_box_sel().0,
                };
            } else {
                tab.mbar.set_err(&format!("{}", &LANG.no_sel_range));
            }
            Log::macros(MacrosFunc::GetSelectedString, &sel_str);
            rv.set(v8::String::new(scope, &sel_str).unwrap().into());
        }
    }
}
