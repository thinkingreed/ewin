use crate::{global_term::*, model::*, terms::term::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::{
    files::file::*,
    model::{ActType, DParts},
};
use std::path::Path;
use v8::{inspector::*, Context, ContextScope, Function, FunctionCallback, HandleScope, Isolate, Local, MapFnTo, Object, Script, TryCatch, V8, V8::*};

impl Macros {
    pub fn exec_js_macro(term: &mut Terminal, js_filenm: &str) -> ActType {
        Log::info_key("exec_js_macro");
        Log::info("exec js file", &js_filenm);

        let isolate = &mut Isolate::new(Default::default());

        // for console.log
        let mut client = InspectorClient::new();
        let mut inspector = V8Inspector::create(isolate, &mut client);

        let scope = &mut HandleScope::new(isolate);
        let context = Context::new(scope);
        let scope = &mut ContextScope::new(scope, context);

        inspector.context_created(context, 1, StringView::empty());

        // Store tab information in global variable
        let _ = EDITOR.get().unwrap().try_lock().map(|mut editor| *editor = term.tabs[term.tab_idx].editor.clone());

        Macros::regist_js_func(scope, context);
        let (script_str, err_str) = File::read_external_file(js_filenm);

        if err_str.is_empty() {
            let code = v8::String::new(scope, &script_str).unwrap();

            let mut scope = v8::TryCatch::new(scope);

            let filename = v8::String::new(&mut scope, js_filenm).unwrap();
            let undefined = v8::undefined(&mut scope);
            let origin = v8::ScriptOrigin::new(&mut scope, filename.into(), 0, 0, false, 0, undefined.into(), false, false, false);

            let script = if let Some(script) = Script::compile(&mut scope, code, Some(&origin)) {
                script
            } else {
                Macros::log_exceptions(scope);
                return ActType::Draw(DParts::MsgBar(format!("{} {}", &Lang::get().script_compile_error, &Lang::get().check_log_file)));
            };
            if let Some(result) = script.run(&mut scope) {
                Log::debug("script.run result", &result.to_string(&mut scope).unwrap().to_rust_string_lossy(&mut scope));

                if let Some(Ok(editor)) = EDITOR.get().map(|tab| tab.try_lock()) {
                    Log::debug("editor", &editor);
                    term.tabs[term.tab_idx].editor = editor.clone();
                }
            } else {
                Macros::log_exceptions(scope);
                return ActType::Draw(DParts::MsgBar(format!("{} {}", &Lang::get().script_run_error, &Lang::get().check_log_file)));
            };
            return ActType::Draw(DParts::All);
        } else {
            return ActType::Draw(DParts::MsgBar(err_str));
        }
    }

    fn log_exceptions(mut try_catch: TryCatch<HandleScope>) {
        let exception = try_catch.exception().unwrap();
        let exception_string = exception.to_string(&mut try_catch).unwrap().to_rust_string_lossy(&mut try_catch);
        let message = if let Some(message) = try_catch.message() {
            message
        } else {
            Log::error_s(&exception_string);
            return;
        };

        // Output (filename):(line number): (message).
        let filepath = message.get_script_resource_name(&mut try_catch).map_or_else(|| "(unknown)".into(), |s| s.to_string(&mut try_catch).unwrap().to_rust_string_lossy(&mut try_catch));
        let line_number = message.get_line_number(&mut try_catch).unwrap_or_default();

        let filenm = Path::new(&filepath).file_name().unwrap().to_string_lossy().to_string();
        Log::error_s(&format!("{}:{}:{}", filenm, line_number, exception_string));

        // Output line of source code.
        let source_line = message.get_source_line(&mut try_catch).map(|s| s.to_string(&mut try_catch).unwrap().to_rust_string_lossy(&mut try_catch)).unwrap();
        Log::error_s(&source_line);

        // Output wavy underline (GetUnderline is deprecated).
        let start_column = message.get_start_column();
        let end_column = message.get_end_column();

        Log::error_s(&format!("{}{}", &" ".repeat(if start_column == 0 { 0 } else { start_column }), &"^".repeat(end_column - start_column)));

        // Output stack trace
        let stack_trace = if let Some(stack_trace) = try_catch.stack_trace() {
            stack_trace
        } else {
            return;
        };
        let stack_trace = unsafe { v8::Local::<v8::String>::cast(stack_trace) };
        let stack_trace = stack_trace.to_string(&mut try_catch).map(|s| s.to_rust_string_lossy(&mut try_catch));
        if let Some(stack_trace) = stack_trace {
            Log::error_s(&stack_trace);
        }
    }

    pub fn init_js_engine() {
        V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
        V8::initialize();
    }
    pub fn exit_js_engine() {
        unsafe {
            v8::V8::dispose();
        }
        v8::V8::dispose_platform();
    }
    pub fn set_data_property(scope: &mut v8::ContextScope<v8::HandleScope>, context: Local<Context>, key: String, func: impl MapFnTo<FunctionCallback>) {
        let global: Local<Object> = context.global(scope);
        let key = v8::String::new(scope, &key).unwrap();
        let func = Function::new(scope, func).unwrap();
        global.create_data_property(scope, key.into(), func.into()).unwrap();
    }
}

struct InspectorClient(V8InspectorClientBase);
impl InspectorClient {
    fn new() -> Self {
        Self(V8InspectorClientBase::new::<Self>())
    }
}

impl V8InspectorClientImpl for InspectorClient {
    fn base(&self) -> &V8InspectorClientBase {
        &self.0
    }
    fn base_mut(&mut self) -> &mut V8InspectorClientBase {
        &mut self.0
    }
    fn console_api_message(&mut self, _context_group_id: i32, _level: i32, message: &StringView, _url: &StringView, _line_number: u32, _column_number: u32, _stack_trace: &mut V8StackTrace) {
        // Log message output
        Log::debug("console.message", &message);
    }
}
