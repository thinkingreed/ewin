use crate::model::*;
use ewin_cfg::model::modal::FileType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use ewin_const::def::*;
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Keybind {
    pub key: String,
    pub cmd: String,
    pub when: String,
}

impl KeyCmd {
    pub fn cmd_when_to_keycmd(keycmd_str: &str, keywhen_str: &str) -> KeyCmd {
        /*
         * All
         */

        return match keywhen_str {
            "headerBarFocus" => KeyCmd::Unsupported,
            "promptFocus" => match keycmd_str {
                /*
                 * Input
                 */
                "cursorLeft" => KeyCmd::Prom(P_Cmd::CursorLeft),
                "cursorRight" => KeyCmd::Prom(P_Cmd::CursorRight),
                "cursorUp" => KeyCmd::Prom(P_Cmd::CursorUp),
                "cursorDown" => KeyCmd::Prom(P_Cmd::CursorDown),
                "cursorRowHome" => KeyCmd::Prom(P_Cmd::CursorRowHome),
                "cursorRowEnd" => KeyCmd::Prom(P_Cmd::CursorRowEnd),
                "cursorLeftSelect" => KeyCmd::Prom(P_Cmd::CursorLeftSelect),
                "cursorRightSelect" => KeyCmd::Prom(P_Cmd::CursorRightSelect),
                "cursorRowHomeSelect" => KeyCmd::Prom(P_Cmd::CursorRowHomeSelect),
                "cursorRowEndSelect" => KeyCmd::Prom(P_Cmd::CursorRowEndSelect),
                "deleteNextChar" => KeyCmd::Prom(P_Cmd::DelNextChar),
                "deletePrevChar" => KeyCmd::Prom(P_Cmd::DelPrevChar),
                "paste" => KeyCmd::Prom(P_Cmd::InsertStr("".to_string())),
                "cutSelect" => KeyCmd::Prom(P_Cmd::Cut),
                "copySelect" => KeyCmd::Prom(P_Cmd::Copy),
                "undo" => KeyCmd::Prom(P_Cmd::Undo),
                "redo" => KeyCmd::Prom(P_Cmd::Redo),
                // Find
                "findNextProm" => KeyCmd::Prom(P_Cmd::FindNext),
                "findBackProm" => KeyCmd::Prom(P_Cmd::FindBack),
                /*
                 * Prompt
                 */
                "escPrompt" => KeyCmd::Prom(P_Cmd::Cancel),
                "confirmPrompt" => KeyCmd::Prom(P_Cmd::Confirm),
                "nextContent" => KeyCmd::Prom(P_Cmd::NextContent),
                "backContent" => KeyCmd::Prom(P_Cmd::BackContent),
                "findCaseSensitive" => KeyCmd::Prom(P_Cmd::FindCaseSensitive),
                "findRegex" => KeyCmd::Prom(P_Cmd::FindRegex),
                /*
                 * Other
                 */
                "closeFile" => KeyCmd::Prom(P_Cmd::CloseFile),
                _ => KeyCmd::Unsupported,
            },
            "editorFocus" => {
                let cmd = KeyCmd::to_edit_cmd(keycmd_str);
                if cmd == E_Cmd::Null {
                    KeyCmd::Unsupported
                } else {
                    KeyCmd::Edit(cmd)
                }
            }
            // unreachable
            "inputFocus" | "allFocus" => KeyCmd::Null,
            _ => KeyCmd::Null,
        };
    }

    pub fn to_edit_cmd(keycmd_str: &str) -> E_Cmd {
        match keycmd_str {
            // Find
            "findNext" => E_Cmd::FindNext,
            "findBack" => E_Cmd::FindBack,
            /*
             * Input
             */
            // cursor move
            "cursorLeft" => E_Cmd::CursorLeft,
            "cursorRight" => E_Cmd::CursorRight,
            "cursorUp" => E_Cmd::CursorUp,
            "cursorDown" => E_Cmd::CursorDown,
            "cursorRowHome" => E_Cmd::CursorRowHome,
            "cursorRowEnd" => E_Cmd::CursorRowEnd,
            "cursorLeftSelect" => E_Cmd::CursorLeftSelect,
            "cursorRightSelect" => E_Cmd::CursorRightSelect,
            "cursorUpSelect" => E_Cmd::CursorUpSelect,
            "cursorDownSelect" => E_Cmd::CursorDownSelect,
            "cursorRowHomeSelect" => E_Cmd::CursorRowHomeSelect,
            "cursorRowEndSelect" => E_Cmd::CursorRowEndSelect,
            "deleteNextChar" => E_Cmd::DelNextChar,
            "deletePrevChar" => E_Cmd::DelPrevChar,
            "paste" => E_Cmd::InsertStr("".to_string()),
            "cutSelect" => E_Cmd::Cut,
            "copySelect" => E_Cmd::Copy,
            "undo" => E_Cmd::Undo,
            "redo" => E_Cmd::Redo,
            /*
             * Editor
             */
            "cursorFileHome" => E_Cmd::CursorFileHome,
            "cursorFileEnd" => E_Cmd::CursorFileEnd,
            "cursorPageUp" => E_Cmd::CursorPageUp,
            "cursorPageDown" => E_Cmd::CursorPageDown,
            // select
            "allSelect" => E_Cmd::AllSelect,
            "boxSelectModeStart" => E_Cmd::BoxSelectMode,
            // edit
            "insertLine" => E_Cmd::InsertRow,
            "formatJSON" => E_Cmd::Format(FileType::JSON),
            "formatXML" => E_Cmd::Format(FileType::XML),
            "formatHTML" => E_Cmd::Format(FileType::HTML),

            // prompt
            "find" => E_Cmd::Find,
            "replace" => E_Cmd::ReplacePrompt,
            "moveLine" => E_Cmd::MoveRow,
            "grep" => E_Cmd::Grep,
            // file
            "closeFile" => E_Cmd::CloseFile,
            "createNewFile" => E_Cmd::CreateNewFile,
            "openFile" => E_Cmd::OpenFile(OpenFileType::Normal),
            "encoding" => E_Cmd::Encoding,
            "closeAllFile" => E_Cmd::CloseAllFile,
            "saveFile" => E_Cmd::SaveFile,
            // key macro
            "startEndRecordKey" => E_Cmd::StartEndRecordKey,
            "execRecordKey" => E_Cmd::ExecRecordKey,
            // mouse
            "mouseOpeSwitch" => E_Cmd::MouseModeSwitch,
            // menu
            "help" => E_Cmd::Help,
            "openMenuFile" => E_Cmd::OpenMenuFile,
            "openMenuConvert" => E_Cmd::OpenMenuConvert,
            "openMenuEdit" => E_Cmd::OpenMenuEdit,
            "openMenuSearch" => E_Cmd::OpenMenuSearch,
            // mode
            "cancelMode" => E_Cmd::CancelState,
            // ContextMenu
            "contextMenu" => E_Cmd::CtxtMenu(USIZE_UNDEFINED, USIZE_UNDEFINED),
            // switchTab
            "switchTabLeft" => E_Cmd::SwitchTabLeft,
            "switchTabRight" => E_Cmd::SwitchTabRight,
            // Input Complement
            "inputComplement" => E_Cmd::InputComple,
            /*
             * Display
             */
            "scale" => E_Cmd::SwitchDispScale,
            "rowNo" => E_Cmd::SwitchDispRowNo,
            _ => E_Cmd::Null,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyCmd {
    Edit(E_Cmd),
    Prom(P_Cmd),
    MenuBar(M_Cmd),
    CtxMenu(C_Cmd),
    FileBar(F_Cmd),
    StatusBar(S_Cmd),
    Unsupported,
    Null,
}

impl Default for KeyCmd {
    fn default() -> Self {
        KeyCmd::Null
    }
}

#[derive(PartialEq, PartialOrd, Eq, Hash, Ord, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum P_Cmd {
    // All
    FindNext,
    FindBack,
    // input forcus
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    CursorRowHome,
    CursorRowEnd,
    CursorLeftSelect,
    CursorRightSelect,
    CursorRowHomeSelect,
    CursorRowEndSelect,
    MouseMove(usize, usize),
    MouseDownLeft(usize, usize),
    MouseDragLeft(usize, usize),
    MouseScrollUp,
    MouseScrollDown,
    // Internal use as an alternative to paste
    InsertStr(String),
    DelPrevChar,
    DelNextChar,
    Cut,
    Copy,
    Undo,
    Redo,
    // other
    NextContent,
    BackContent,
    Cancel,
    Confirm,
    FindCaseSensitive,
    FindRegex,
    Resize(usize, usize),
    CloseFile,
    Null,
}

impl Default for P_Cmd {
    fn default() -> Self {
        P_Cmd::Null
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum E_Cmd {
    FindNext,
    FindBack,
    // Input furcus
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
    MouseDownLeft(usize, usize),
    MouseUpLeft(usize, usize),
    MouseDragLeftLeft(usize, usize),
    MouseDragLeftRight(usize, usize),
    MouseDragLeftUp(usize, usize),
    MouseDragLeftDown(usize, usize),
    MouseScrollUp,
    MouseScrollDown,
    DelPrevChar,
    DelNextChar,
    Cut,
    Copy,
    Undo,
    Redo,
    // cursor move
    CursorFileHome,
    CursorFileEnd,
    CursorPageDown,
    CursorPageUp,
    // select
    AllSelect,
    // edit
    InsertStr(String),
    // For Box Insert redo
    InsertBox(Vec<(SelRange, String)>),
    DelBox(Vec<(SelRange, String)>),
    InsertRow,
    Format(FileType),
    // find
    Find,
    ReplaceExec(String, String, BTreeSet<usize>),
    ReplacePrompt,
    MoveRow,
    Grep,
    GrepResult,
    Encoding,
    // file
    CreateNewFile,
    OpenFile(OpenFileType),
    CloseFile,
    CloseAllFile,
    SaveFile,
    // key record
    StartEndRecordKey,
    ExecRecordKey,
    // mouse
    MouseMove(usize, usize),
    MouseDownLeftBox(usize, usize),
    MouseDragLeftBox(usize, usize),
    MouseModeSwitch,
    // menu
    Help,
    OpenMenuFile,
    OpenMenuConvert,
    OpenMenuEdit,
    OpenMenuSearch,
    OpenMenuMacro,
    CtxtMenu(usize, usize),
    // mode
    BoxSelectMode,
    CancelState,
    ReOpenFile,
    // SwitchTab
    SwitchTabRight,
    SwitchTabLeft,
    // Other
    InputComple,
    // InputCompleConfirm,
    Resize(usize, usize),
    // Display
    SwitchDispScale,
    SwitchDispRowNo,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
// Menubar
pub enum M_Cmd {
    MouseMove(usize, usize),
    MouseDownLeft(usize, usize),
    MenuWidget(usize, usize),
    CursorDown,
    CursorUp,
    CursorRight,
    CursorLeft,
    Confirm,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
// Ctx_menu
pub enum C_Cmd {
    MouseMove(usize, usize),
    MouseDownLeft(usize, usize),
    CtxMenu(usize, usize),
    CursorDown,
    CursorUp,
    CursorRight,
    CursorLeft,
    Confirm,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
// Filebar
pub enum F_Cmd {
    MouseDownLeft(usize, usize),
    MouseUpLeft(usize, usize),
    MouseDragLeftUp(usize, usize),
    MouseDragLeftDown(usize, usize),
    MouseDragLeftRight(usize, usize),
    MouseDragLeftLeft(usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
// Statusbar
pub enum S_Cmd {
    MouseDownLeft(usize, usize),
}
