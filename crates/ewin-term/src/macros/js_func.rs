use crate::{ewin_com::model::*, global_term::TAB, model::*};
use ewin_cfg::{log::*, model::modal::*};

use ewin_com::_cfg::key::cmd::{Cmd, CmdType};
use rusty_v8::{self as v8, FunctionCallbackArguments, HandleScope, ReturnValue};
use v8::{Context, Local};

impl Macros {
    pub fn regist_js_func(scope: &mut v8::ContextScope<v8::HandleScope>, context: Local<Context>) {
        Macros::set_data_property(scope, context, MacrosFunc::insertString.to_string(), Macros::insertString);
        Macros::set_data_property(scope, context, MacrosFunc::getSelectedString.to_string(), Macros::getSelectedString);
        Macros::set_data_property(scope, context, MacrosFunc::getAllString.to_string(), Macros::getAllString);
        Macros::set_data_property(scope, context, MacrosFunc::searchAll.to_string(), Macros::searchAll);
    }

    /*
     * Edit
     */
    #[allow(non_snake_case)]
    pub fn insertString(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
        // If Insert Text is not empty
        if args.get(0) != v8::undefined(scope) {
            let input_str = args.get(0).to_string(scope).unwrap();

            if let Some(Ok(mut tab)) = TAB.get().map(|tab| tab.try_lock()) {
                tab.editor.edit_proc(Cmd::to_cmd(CmdType::InsertStr(input_str.to_rust_string_lossy(scope))));
                tab.editor.state.is_changed = true;
                Log::macros(MacrosFunc::insertString, &input_str.to_rust_string_lossy(scope));
            }
        }
    }
    /*
     * GetString
     */
    #[allow(non_snake_case)]
    pub fn getSelectedString(scope: &mut HandleScope, _: FunctionCallbackArguments, mut rv: ReturnValue) {
        if let Some(Ok(mut tab)) = TAB.get().map(|tab| tab.try_lock()) {
            let mut sel_str = String::new();
            Log::debug("tab.editor.sel", &tab.editor.win_mgr.curt().sel);
            if tab.editor.win_mgr.curt().sel.is_selected() {
                sel_str = match tab.editor.win_mgr.curt().sel.mode {
                    SelMode::Normal => tab.editor.buf.slice(tab.editor.win_mgr.curt_ref().sel.get_range()),
                    SelMode::BoxSelect => tab.editor.slice_box_sel().0,
                };
            }
            Log::macros(MacrosFunc::getSelectedString, &sel_str);
            rv.set(v8::String::new(scope, &sel_str).unwrap().into());
        }
    }

    #[allow(non_snake_case)]
    pub fn getAllString(scope: &mut HandleScope, _: FunctionCallbackArguments, mut rv: ReturnValue) {
        if let Some(Ok(tab)) = TAB.get().map(|tab| tab.try_lock()) {
            Log::macros(MacrosFunc::getSelectedString, &"");

            let string = &tab.editor.buf.text.to_string();
            rv.set(v8::String::new(scope, &string[..string.chars().count() - 2]).unwrap().into());
        }
    }
    /*
     * Search
     */

    #[allow(non_snake_case)]
    pub fn searchAll(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
        Log::debug_key("Macros.searchAll");

        if args.get(0) != v8::undefined(scope) {
            let search_str = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
            let input_obj = args.get(1).to_object(scope).unwrap();

            let case_sens_value: v8::Local<v8::Value> = v8::String::new(scope, "caseSens").unwrap().into();
            let case_sens = input_obj.get(scope, case_sens_value).unwrap().boolean_value(scope);

            let regex_value: v8::Local<v8::Value> = v8::String::new(scope, "regex").unwrap().into();
            let regex = input_obj.get(scope, regex_value).unwrap().boolean_value(scope);

            if let Some(Ok(tab)) = TAB.get().map(|tab| tab.try_lock()) {
                // TODO
                // tab.editor.search(&search_str, &CfgSearch { regex, case_sensitive: case_sens });
                Log::macros(MacrosFunc::searchAll, &"");
            }
        }
    }
}
