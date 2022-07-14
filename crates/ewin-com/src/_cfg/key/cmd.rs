use super::keys::{Key, *};
use crate::{_cfg::key::keywhen::*, global::*, model::*, util::*};
use ewin_cfg::model::modal::*;
use ewin_const::def::*;
use std::{cmp::Ordering, collections::BTreeSet};

impl Cmd {
    pub fn keys_to_cmd(keys: &Keys, keys_org_opt: Option<&Keys>, keywhen: KeyWhen) -> Cmd {
        let result = CMD_MAP.get().unwrap().get(&(*keys, KeyWhen::All)).or_else(|| CMD_MAP.get().unwrap().get(&(*keys, keywhen.clone())));

        let cmd = match result {
            Some(cmd) => cmd.clone(),
            None => Cmd::to_cmd(CmdType::Null),
        };
        if cmd.cmd_type != CmdType::Null {
            return cmd;
        }
        // common
        let cmd_type = match &keys {
            Keys::Resize(x, y) => CmdType::Resize(*x as usize, *y as usize),
            Keys::Raw(Key::Char(c)) => CmdType::InsertStr(c.to_string()),
            Keys::Shift(Key::Char(c)) => CmdType::InsertStr(c.to_ascii_uppercase().to_string()),
            Keys::MouseMove(y, x) => CmdType::MouseMove(*y as usize, *x as usize),
            Keys::MouseAltDownLeft(y, x) => CmdType::MouseDownLeftBox(*y as usize, *x as usize),
            Keys::MouseAltDragLeft(y, x) => CmdType::MouseDragLeftBox(*y as usize, *x as usize),
            Keys::MouseDownLeft(y, x) => CmdType::MouseDownLeft(*y as usize, *x as usize),
            Keys::MouseUpLeft(y, x) => CmdType::MouseUpLeft(*y as usize, *x as usize),
            Keys::MouseScrollUp => CmdType::MouseScrollUp,
            Keys::MouseScrollDown => CmdType::MouseScrollDown,

            Keys::MouseDragLeft(y, x) => {
                match keys_org_opt {
                    Some(Keys::MouseDragLeft(y_org, x_org)) | Some(Keys::MouseDownLeft(y_org, x_org)) => match y.cmp(y_org) {
                        Ordering::Less => CmdType::MouseDragLeftUp(*y as usize, *x as usize),
                        Ordering::Greater => CmdType::MouseDragLeftDown(*y as usize, *x as usize),
                        Ordering::Equal => {
                            if x > x_org || x == &(get_term_size().0 as u16) {
                                CmdType::MouseDragLeftRight(*y as usize, *x as usize)
                            } else {
                                CmdType::MouseDragLeftLeft(*y as usize, *x as usize)
                            }
                        }
                    },
                    // dummy
                    _ => CmdType::MouseDragLeftDown(*y as usize, *x as usize),
                }
            }
            Keys::MouseDownRight(y, x) | Keys::MouseDragRight(y, x) => CmdType::CtxtMenu(*y as usize, *x as usize),

            _ => CmdType::Null,
        };

        if cmd_type != CmdType::Null {
            return Cmd::to_cmd(cmd_type);
        }

        let cmd_type = match keywhen {
            KeyWhen::Editor => match &keys {
                Keys::Raw(Key::Tab) => CmdType::InsertStr(TAB_CHAR.to_string()),
                _ => CmdType::Unsupported,
            },
            /*
            KeyWhen::Prom => match &keys {
                Keys::Raw(Key::Tab) => CmdType::NextContent,
                Keys::Raw(Key::BackTab) => CmdType::BackContent,
                _ => CmdType::Unsupported,
            },
            */
            _ => CmdType::Unsupported,
        };
        return Cmd::to_cmd(cmd_type);
    }

    pub fn str_to_cmd(keycmd_str: &str) -> Cmd {
        return Cmd::to_cmd(Cmd::str_to_cmd_type(keycmd_str));
    }

    pub fn str_to_cmd_type(keycmd_str: &str) -> CmdType {
        let cmd_type = match keycmd_str {
            /*
             * Input
             */
            "undo" => CmdType::Undo,
            "redo" => CmdType::Redo,
            "deleteNextChar" => CmdType::DelNextChar,
            "deletePrevChar" => CmdType::DelPrevChar,

            "cursorLeft" => CmdType::CursorLeft,
            "cursorRight" => CmdType::CursorRight,
            "cursorUp" => CmdType::CursorUp,
            "cursorDown" => CmdType::CursorDown,
            "cursorRowHome" => CmdType::CursorRowHome,
            "cursorRowEnd" => CmdType::CursorRowEnd,
            "cursorLeftSelect" => CmdType::CursorLeftSelect,
            "cursorRightSelect" => CmdType::CursorRightSelect,
            "cursorUpSelect" => CmdType::CursorUpSelect,
            "cursorDownSelect" => CmdType::CursorDownSelect,
            "cursorRowHomeSelect" => CmdType::CursorRowHomeSelect,
            "cursorRowEndSelect" => CmdType::CursorRowEndSelect,
            "cursorFileHome" => CmdType::CursorFileHome,
            "cursorFileEnd" => CmdType::CursorFileEnd,
            "cursorPageUp" => CmdType::CursorPageUp,
            "cursorPageDown" => CmdType::CursorPageDown,

            "paste" => CmdType::InsertStr("".to_string()),
            "cut" => CmdType::Cut,
            "copy" => CmdType::Copy,
            "findNext" => CmdType::FindNext,
            "findBack" => CmdType::FindBack,
            "cancelPrompt" => CmdType::CancelProm,
            "confirmPrompt" => CmdType::Confirm,
            "nextContent" => CmdType::NextContent,
            "backContent" => CmdType::BackContent,
            "findCaseSensitive" => CmdType::FindCaseSensitive,
            "findRegex" => CmdType::FindRegex,
            "closeFile" => CmdType::CloseFile,

            // editor
            "allSelect" => CmdType::AllSelect,
            "boxSelectModeStart" => CmdType::BoxSelectModeStart,
            "insertLine" => CmdType::InsertRow,
            "createNewFile" => CmdType::CreateNewFile,
            "closeAllFile" => CmdType::CloseAllFile,
            "saveFile" => CmdType::SaveFile,
            "save_as" => CmdType::SaveNewFile,
            "formatJSON" => CmdType::Format(FileType::JSON),
            "formatXML" => CmdType::Format(FileType::XML),
            "formatHTML" => CmdType::Format(FileType::HTML),

            // key macro
            "startEndRecordKey" => CmdType::RecordKeyStartEnd,
            "execRecordKey" => CmdType::ExecRecordKey,
            // mouse
            "mouseModeSwitch" => CmdType::MouseModeSwitch,
            // menu
            "help" => CmdType::Help,
            "helpInitDisplaySwitch" => CmdType::HelpInitDisplaySwitch,
            "openMenuFile" => CmdType::OpenMenuFile,
            "openMenuConvert" => CmdType::OpenMenuConvert,
            "openMenuEdit" => CmdType::OpenMenuEdit,
            "openMenuSearch" => CmdType::OpenMenuSearch,
            "openMenuMacro" => CmdType::OpenMenuSearch,
            // switchTab
            "switchTabLeft" => CmdType::SwitchTab(Direction::Left),
            "switchTabRight" => CmdType::SwitchTab(Direction::Right),
            // mode
            "cancelEditorState" => CmdType::CancelEditorState,
            // ContextMenu
            "contextMenu" => CmdType::CtxtMenu(USIZE_UNDEFINED, USIZE_UNDEFINED),
            // Input Complement
            "inputComplement" => CmdType::InputComple,
            /*
             * Display
             */
            "scale" => CmdType::SwitchDispScale,
            "rowNo" => CmdType::SwitchDispRowNo,
            // prom
            "findPrompt" => CmdType::FindProm,
            "replacePrompt" => CmdType::ReplaceProm,
            "moveLinePrompt" => CmdType::MoveRowProm,
            "grepPrompt" => CmdType::GrepProm,
            "encodingPrompt" => CmdType::EncodingProm,
            "openFilePrompt" => CmdType::openFileProm(OpenFileType::Normal),
            // window
            "windowSplitVertical" => CmdType::WindowSplit(WindowSplitType::Vertical),
            "windowSplitHorizontal" => CmdType::WindowSplit(WindowSplitType::Horizontal),
            _ => CmdType::Unsupported,
        };

        return cmd_type;
    }

    pub fn to_cmd(cmd_type: CmdType) -> Cmd {
        return match cmd_type {
            // edit
            CmdType::InsertStr(_) | CmdType::Cut | CmdType::InsertRow | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo | CmdType::InsertBox(_) | CmdType::DelBox(_) | CmdType::ReplaceExec(_, _, _) | CmdType::Format(_) => Cmd { cmd_type, when_vec: vec![KeyWhen::Editor, KeyWhen::Prom], config: CmdConfig { is_edit: true, is_record: true, is_recalc_scrl: true } },
            // key macro
            CmdType::RecordKeyStartEnd => Cmd { cmd_type, ..Cmd::default() },
            CmdType::ExecRecordKey => Cmd { cmd_type, when_vec: vec![KeyWhen::Editor], config: CmdConfig { is_edit: true, is_recalc_scrl: true, ..CmdConfig::default() } },
            // Cursor move Editor and Prom
            CmdType::CursorLeft | CmdType::CursorRight | CmdType::CursorUp | CmdType::CursorDown | CmdType::CursorRowHome | CmdType::CursorRowEnd | CmdType::CursorLeftSelect | CmdType::CursorRightSelect | CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect => Cmd { cmd_type, when_vec: vec![KeyWhen::All], config: CmdConfig { is_record: true, is_recalc_scrl: true, ..CmdConfig::default() } },
            // Cursor move Editor only
            CmdType::CursorUpSelect | CmdType::CursorDownSelect | CmdType::CursorFileHome | CmdType::CursorFileEnd | CmdType::CursorPageUp | CmdType::CursorPageDown => Cmd { cmd_type, when_vec: vec![KeyWhen::Editor], config: CmdConfig { is_record: true, is_recalc_scrl: true, ..CmdConfig::default() } },
            // select
            CmdType::Copy | CmdType::AllSelect => Cmd { cmd_type, when_vec: vec![KeyWhen::Editor, KeyWhen::Prom], config: CmdConfig { is_record: true, ..CmdConfig::default() } },
            // mouse
            CmdType::MouseDownLeft(_, _) | CmdType::MouseUpLeft(_, _) | CmdType::MouseMove(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseDownLeftBox(_, _) | CmdType::MouseDragLeftBox(_, _) | CmdType::MouseScrollUp | CmdType::MouseScrollDown => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, ..Cmd::default() },
            CmdType::MouseModeSwitch => Cmd { cmd_type, ..Cmd::default() },

            CmdType::BoxSelectMode => Cmd { cmd_type, ..Cmd::default() },
            CmdType::BoxSelectModeStart => Cmd { cmd_type, ..Cmd::default() },
            // File
            CmdType::CloseFile => Cmd { cmd_type, when_vec: vec![KeyWhen::All], ..Cmd::default() },
            CmdType::CloseAllFile => Cmd { cmd_type, ..Cmd::default() },
            CmdType::ReOpenFile => Cmd { cmd_type, ..Cmd::default() },
            // menu
            CmdType::HelpInitDisplaySwitch => Cmd { cmd_type, ..Cmd::default() },
            CmdType::OpenMenuFile | CmdType::OpenMenuConvert | CmdType::OpenMenuEdit | CmdType::OpenMenuSearch | CmdType::OpenMenuMacro => Cmd { cmd_type, ..Cmd::default() },
            // ContextMenu
            CmdType::CtxtMenu(_, _) => Cmd { cmd_type, when_vec: vec![KeyWhen::All], ..Cmd::default() },
            /*
             * Editor
             */
            CmdType::CreateNewFile | CmdType::InputComple | CmdType::CancelEditorState | CmdType::SaveFile | CmdType::SwitchTab(_) | CmdType::Help => Cmd { cmd_type, when_vec: vec![KeyWhen::Editor], ..Cmd::default() },
            CmdType::WindowSplit(_) => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, when_vec: vec![KeyWhen::Editor] },
            /*
             * Display
             */
            CmdType::SwitchDispScale | CmdType::SwitchDispRowNo => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, ..Cmd::default() },
            // Prom
            CmdType::FindProm | CmdType::ReplaceProm | CmdType::MoveRowProm | CmdType::GrepProm | CmdType::GrepingProm | CmdType::GrepResultProm | CmdType::EncodingProm | CmdType::openFileProm(_) | CmdType::Saveforced | CmdType::WatchFileResult => Cmd { cmd_type, when_vec: vec![KeyWhen::Editor], ..Cmd::default() },
            CmdType::SaveNewFile => Cmd { cmd_type, when_vec: vec![KeyWhen::MenuBar], ..Cmd::default() },
            CmdType::FindNext | CmdType::FindBack => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, when_vec: vec![KeyWhen::Editor, KeyWhen::Prom] },
            CmdType::FindCaseSensitive | CmdType::FindRegex => Cmd { cmd_type, when_vec: vec![KeyWhen::Prom], ..Cmd::default() },

            CmdType::Confirm => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, when_vec: vec![KeyWhen::MenuBar, KeyWhen::Prom, KeyWhen::CtxMenu] },
            CmdType::CancelProm => Cmd { cmd_type, when_vec: vec![KeyWhen::Prom], ..Cmd::default() },
            CmdType::NextContent | CmdType::BackContent => Cmd { cmd_type, when_vec: vec![KeyWhen::Prom], ..Cmd::default() },
            // MenuBar
            CmdType::MenuWidget(_, _) => Cmd { cmd_type, ..Cmd::default() },
            // CtxMenu
            CmdType::CtxMenu(_, _) => Cmd { cmd_type, ..Cmd::default() },
            // Other
            CmdType::Resize(_, _) => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, ..Cmd::default() },
            CmdType::Null => Cmd { cmd_type, ..Cmd::default() },
            CmdType::Unsupported => Cmd { cmd_type, ..Cmd::default() },
        };
    }

    pub fn get_cmd_str(cmd_type: &CmdType) -> String {
        let result = CMD_TYPE_MAP.get().unwrap().get(cmd_type);
        match result {
            Some(key) => key.to_string(),
            None => "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Cmd {
    pub cmd_type: CmdType,
    pub config: CmdConfig,
    pub when_vec: Vec<KeyWhen>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum CmdType {
    /*
     * Prom
     */
    InsertStr(String),
    Cut,
    DelNextChar,
    DelPrevChar,
    Copy,
    Undo,
    Redo,
    InsertBox(Vec<(SelRange, String)>),
    DelBox(Vec<(SelRange, String)>),
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    CursorRowHome,
    CursorRowEnd,
    CursorLeftSelect,
    CursorRightSelect,
    CursorUpSelect,
    CursorDownSelect,
    CursorRowHomeSelect,
    CursorRowEndSelect,
    CursorFileHome,
    CursorFileEnd,
    AllSelect,
    CursorPageUp,
    CursorPageDown,
    FindNext,
    FindBack,
    MouseDownLeft(usize, usize),
    MouseUpLeft(usize, usize),
    MouseDragLeftUp(usize, usize),
    MouseDragLeftDown(usize, usize),
    MouseDragLeftLeft(usize, usize),
    MouseDragLeftRight(usize, usize),
    MouseDownLeftBox(usize, usize),
    MouseDragLeftBox(usize, usize),
    // MouseDragLeft(usize, usize),
    MouseMove(usize, usize),
    MouseScrollUp,
    MouseScrollDown,
    Resize(usize, usize),
    Confirm,
    CancelProm,
    NextContent,
    BackContent,
    FindCaseSensitive,
    FindRegex,
    ReOpenFile,
    CloseFile,
    Unsupported,
    // Editor
    BoxSelectModeStart,
    BoxSelectMode,
    InsertRow,
    CreateNewFile,
    CloseAllFile,
    SaveFile,
    Format(FileType),
    RecordKeyStartEnd,
    ExecRecordKey,
    MouseModeSwitch,
    Help,
    HelpInitDisplaySwitch,
    OpenMenuFile,
    OpenMenuConvert,
    OpenMenuEdit,
    OpenMenuSearch,
    OpenMenuMacro,
    SwitchTab(Direction),
    CancelEditorState,
    CtxtMenu(usize, usize),
    InputComple,
    SwitchDispScale,
    SwitchDispRowNo,

    // Prom
    FindProm,
    ReplaceProm,
    ReplaceExec(String, String, BTreeSet<usize>),
    MoveRowProm,
    GrepProm,
    GrepingProm,
    GrepResultProm,
    EncodingProm,
    openFileProm(OpenFileType),
    SaveNewFile,
    Saveforced,
    WatchFileResult,
    // MenuBar
    MenuWidget(usize, usize),
    // CtxMenu
    CtxMenu(usize, usize),
    // Window
    WindowSplit(WindowSplitType),
    #[default]
    Null,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CmdConfig {
    pub is_edit: bool,
    pub is_record: bool,
    pub is_recalc_scrl: bool,
}
