use crate::model::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Keybind {
    pub key: String,
    pub cmd: String,
    pub when: String,
}

impl KeyCmd {
    pub fn cmd_when_to_keycmd(keycmd: &str, keywhen: &str) -> KeyCmd {
        /*
         * All
         */

        if keycmd == "closeFile" {
            return KeyCmd::CloseFile;
        }

        match keywhen {
            "promptFocus" => match keycmd {
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
                "findNext" => KeyCmd::Prom(P_Cmd::FindNext),
                "findBack" => KeyCmd::Prom(P_Cmd::FindBack),
                /*
                 * Prompt
                 */
                "escPrompt" => KeyCmd::Prom(P_Cmd::EscPrompt),
                "confirmPrompt" => KeyCmd::Prom(P_Cmd::ConfirmPrompt),
                "findCaseSensitive" => KeyCmd::Prom(P_Cmd::FindCaseSensitive),
                "findRegex" => KeyCmd::Prom(P_Cmd::FindRegex),
                _ => KeyCmd::Unsupported,
            },
            "editorFocus" => match keycmd {
                // Find
                "findNext" => KeyCmd::Edit(E_Cmd::FindNext),
                "findBack" => KeyCmd::Edit(E_Cmd::FindBack),
                /*
                 * Input
                 */
                // cursor move
                "cursorLeft" => KeyCmd::Edit(E_Cmd::CursorLeft),
                "cursorRight" => KeyCmd::Edit(E_Cmd::CursorRight),
                "cursorUp" => KeyCmd::Edit(E_Cmd::CursorUp),
                "cursorDown" => KeyCmd::Edit(E_Cmd::CursorDown),
                "cursorRowHome" => KeyCmd::Edit(E_Cmd::CursorRowHome),
                "cursorRowEnd" => KeyCmd::Edit(E_Cmd::CursorRowEnd),
                "cursorLeftSelect" => KeyCmd::Edit(E_Cmd::CursorLeftSelect),
                "cursorRightSelect" => KeyCmd::Edit(E_Cmd::CursorRightSelect),
                "cursorUpSelect" => KeyCmd::Edit(E_Cmd::CursorUpSelect),
                "cursorDownSelect" => KeyCmd::Edit(E_Cmd::CursorDownSelect),
                "cursorRowHomeSelect" => KeyCmd::Edit(E_Cmd::CursorRowHomeSelect),
                "cursorRowEndSelect" => KeyCmd::Edit(E_Cmd::CursorRowEndSelect),
                "deleteNextChar" => KeyCmd::Edit(E_Cmd::DelNextChar),
                "deletePrevChar" => KeyCmd::Edit(E_Cmd::DelPrevChar),
                "paste" => KeyCmd::Edit(E_Cmd::InsertStr("".to_string())),
                "cutSelect" => KeyCmd::Edit(E_Cmd::Cut),
                "copySelect" => KeyCmd::Edit(E_Cmd::Copy),
                "undo" => KeyCmd::Edit(E_Cmd::Undo),
                "redo" => KeyCmd::Edit(E_Cmd::Redo),
                /*
                 * Editor
                 */
                "cursorFileHome" => KeyCmd::Edit(E_Cmd::CursorFileHome),
                "cursorFileEnd" => KeyCmd::Edit(E_Cmd::CursorFileEnd),
                "cursorPageUp" => KeyCmd::Edit(E_Cmd::CursorPageUp),
                "cursorPageDown" => KeyCmd::Edit(E_Cmd::CursorPageDown),
                // select
                "allSelect" => KeyCmd::Edit(E_Cmd::AllSelect),
                "boxSelectModeStart" => KeyCmd::Edit(E_Cmd::BoxSelectMode),
                // edit
                "insertLine" => KeyCmd::Edit(E_Cmd::InsertLine),
                "formatJSON" => KeyCmd::Edit(E_Cmd::Format(FmtType::JSON)),
                "formatXML" => KeyCmd::Edit(E_Cmd::Format(FmtType::XML)),
                "formatHTML" => KeyCmd::Edit(E_Cmd::Format(FmtType::HTML)),

                // prompt
                "find" => KeyCmd::Edit(E_Cmd::Find),
                "replace" => KeyCmd::Edit(E_Cmd::ReplacePrompt),
                "moveLine" => KeyCmd::Edit(E_Cmd::MoveRow),
                "grep" => KeyCmd::Edit(E_Cmd::Grep),
                // file
                "newTab" => KeyCmd::Edit(E_Cmd::NewTab),
                "switchTabRight" => KeyCmd::HeaderBar(H_Cmd::SwitchTabRight),
                "switchTabLeft" => KeyCmd::HeaderBar(H_Cmd::SwitchTabLeft),
                "openFile" => KeyCmd::Edit(E_Cmd::OpenFile(OpenFileType::Normal)),
                "encoding" => KeyCmd::Edit(E_Cmd::Encoding),
                "closeAllFile" => KeyCmd::Edit(E_Cmd::CloseAllFile),
                "saveFile" => KeyCmd::Edit(E_Cmd::SaveFile),
                // key macro
                "startEndRecordKey" => KeyCmd::Edit(E_Cmd::StartEndRecordKey),
                "execRecordKey" => KeyCmd::Edit(E_Cmd::ExecRecordKey),
                // mouse
                "mouseOpeSwitch" => KeyCmd::Edit(E_Cmd::MouseModeSwitch),
                // menu
                "help" => KeyCmd::Edit(E_Cmd::Help),
                "openMenu" => KeyCmd::Edit(E_Cmd::OpenMenu),
                "openMenuFile" => KeyCmd::Edit(E_Cmd::OpenMenuFile),
                "openMenuConvert" => KeyCmd::Edit(E_Cmd::OpenMenuConvert),
                "openMenuEdit" => KeyCmd::Edit(E_Cmd::OpenMenuEdit),
                "openMenuSearch" => KeyCmd::Edit(E_Cmd::OpenMenuSearch),
                // mode
                "cancelMode" => KeyCmd::Edit(E_Cmd::CancelMode),
                // ContextMenu
                "contextMenu" => KeyCmd::Edit(E_Cmd::CtxtMenu),
                _ => KeyCmd::Unsupported,
            },

            // unreachable
            "inputFocus" | "allFocus" => KeyCmd::Null,
            _ => KeyCmd::Null,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyCmd {
    Edit(E_Cmd),
    Prom(P_Cmd),
    CtxMenu(C_Cmd),
    HeaderBar(H_Cmd),
    StatusBar(S_Cmd),
    CloseFile,
    Resize,
    Unsupported,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    TabNextFocus,
    BackTabBackFocus,
    EscPrompt,
    ConfirmPrompt,
    FindCaseSensitive,
    FindRegex,
    Null,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum E_Cmd {
    // All
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
    MouseDragLeftLeft(usize, usize),
    MouseDragLeftRight(usize, usize),
    MouseDragLeftUp(usize, usize),
    MouseDragLeftDown(usize, usize),
    MouseDownRight(usize, usize),
    MouseDragRight(usize, usize),
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
    // cursor move
    CursorFileHome,
    CursorFileEnd,
    CursorPageDown,
    CursorPageUp,
    // select
    AllSelect,
    // edit
    // InsertChar(char),
    InsertBox(Vec<(SelRange, String)>),
    DelBox(Vec<(SelRange, String)>),
    InsertLine,
    Format(FmtType),
    // find
    Find,
    ReplaceExec(bool, String, BTreeMap<(usize, usize), String>),
    ReplacePrompt,
    MoveRow,
    Grep,
    GrepResult,
    Encoding,
    // file
    NewTab,
    OpenFile(OpenFileType),
    CloseAllFile,
    SaveFile,
    // key record
    StartEndRecordKey,
    ExecRecordKey,
    // mouse
    MouseMove(usize, usize),
    MouseDownBoxLeft(usize, usize),
    MouseDragBoxLeft(usize, usize),
    MouseModeSwitch,
    // menu
    Help,
    OpenMenu,
    OpenMenuFile,
    OpenMenuConvert,
    OpenMenuEdit,
    OpenMenuSearch,
    OpenMenuMacro,
    CtxtMenu,
    // mode
    BoxSelectMode,
    CancelMode,
    // Other
    // Unsupported,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum C_Cmd {
    MouseMove(usize, usize),
    MouseDownLeft(usize, usize),
    CursorDown,
    CursorUp,
    CursorRight,
    CursorLeft,
    ConfirmCtxMenu,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum H_Cmd {
    MouseDownLeft(usize, usize),
    MouseDragLeftUp(usize, usize),
    SwitchTabRight,
    SwitchTabLeft,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum S_Cmd {
    MouseDownLeft(usize, usize),
}
