use crate::{def::USIZE_UNDEFINED, model::*};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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

        return match keywhen {
            "headerBarFocus" => KeyCmd::Unsupported,
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
                /*
                 * Other
                 */
                "closeFile" => KeyCmd::Prom(P_Cmd::CloseFile),
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
                "insertLine" => KeyCmd::Edit(E_Cmd::InsertRow),
                "formatJSON" => KeyCmd::Edit(E_Cmd::Format(FileType::JSON)),
                "formatXML" => KeyCmd::Edit(E_Cmd::Format(FileType::XML)),
                "formatHTML" => KeyCmd::Edit(E_Cmd::Format(FileType::HTML)),

                // prompt
                "find" => KeyCmd::Edit(E_Cmd::Find),
                "replace" => KeyCmd::Edit(E_Cmd::ReplacePrompt),
                "moveLine" => KeyCmd::Edit(E_Cmd::MoveRow),
                "grep" => KeyCmd::Edit(E_Cmd::Grep),
                // file
                "closeFile" => KeyCmd::Edit(E_Cmd::CloseFile),
                "newTab" => KeyCmd::Edit(E_Cmd::NewTab),
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
                "cancelMode" => KeyCmd::Edit(E_Cmd::CancelState),
                // ContextMenu
                "contextMenu" => KeyCmd::Edit(E_Cmd::CtxtMenu(USIZE_UNDEFINED, USIZE_UNDEFINED)),
                // switchTab
                "switchTabLeft" => KeyCmd::Edit(E_Cmd::SwitchTabLeft),
                "switchTabRight" => KeyCmd::Edit(E_Cmd::SwitchTabRight),
                // Input Complement
                "inputComplement" => KeyCmd::Edit(E_Cmd::InputComple),
                _ => KeyCmd::Unsupported,
            },

            // unreachable
            "inputFocus" | "allFocus" => KeyCmd::Null,
            _ => KeyCmd::Null,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyCmd {
    Edit(E_Cmd),
    Prom(P_Cmd),
    CtxMenu(C_Cmd),
    HeaderBar(H_Cmd),
    StatusBar(S_Cmd),
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
    Resize(usize, usize),
    CloseFile,
    Null,
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
    NewTab,
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
    OpenMenu,
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
    Resize(usize, usize),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum C_Cmd {
    MouseMove(usize, usize),
    MouseDownLeft(usize, usize),
    CtxMenu(usize, usize),
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
    MouseUpLeft(usize, usize),
    MouseDragLeftUp(usize, usize),
    MouseDragLeftDown(usize, usize),
    MouseDragLeftRight(usize, usize),
    MouseDragLeftLeft(usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum S_Cmd {
    MouseDownLeft(usize, usize),
}
