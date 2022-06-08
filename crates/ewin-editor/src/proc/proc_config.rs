use crate::ewin_com::_cfg::key::keycmd::*;

#[derive(Debug, Default, Clone)]
#[allow(non_camel_case_types)]
pub struct E_CmdConfig {
    pub is_edit: bool,
    pub is_record: bool,
}

impl E_CmdConfig {
    pub fn new(e_cmd: &E_Cmd) -> Self {
        return match e_cmd {
            // Edit
            E_Cmd::InsertStr(_) | E_Cmd::InsertRow | E_Cmd::Cut | E_Cmd::DelPrevChar | E_Cmd::DelNextChar | E_Cmd::InsertBox(_) | E_Cmd::DelBox(_) => E_CmdConfig { is_edit: true, is_record: true },
            // Cur move
            E_Cmd::CursorLeft | E_Cmd::CursorRight | E_Cmd::CursorUp | E_Cmd::CursorDown | E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd | E_Cmd::CursorFileHome | E_Cmd::CursorFileEnd | E_Cmd::CursorPageDown | E_Cmd::CursorPageUp | E_Cmd::FindNext | E_Cmd::FindBack => E_CmdConfig { is_edit: false, is_record: true },
            // Cur move select
            E_Cmd::CursorLeftSelect | E_Cmd::CursorRightSelect | E_Cmd::CursorUpSelect | E_Cmd::CursorDownSelect | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect => E_CmdConfig { is_edit: false, is_record: true },
            // Select
            E_Cmd::Copy | E_Cmd::AllSelect => E_CmdConfig { is_edit: false, is_record: true },
            // Prom edit
            E_Cmd::ReplacePrompt | E_Cmd::Encoding => E_CmdConfig { is_edit: true, is_record: false },
            // Other
            E_Cmd::CtxtMenu(_, _) => E_CmdConfig { is_edit: false, is_record: false },

            // Mouse
            E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseUpLeft(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) | E_Cmd::MouseDragLeftUp(_, _) | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::MouseScrollUp | E_Cmd::MouseScrollDown => E_CmdConfig { is_edit: false, is_record: false },
            E_Cmd::Undo | E_Cmd::Redo => E_CmdConfig { is_edit: true, is_record: true },
            // Other
            E_Cmd::Format(_)
            | E_Cmd::Find
            | E_Cmd::ReplaceExec(_, _, _)
            | E_Cmd::MoveRow
            | E_Cmd::Grep
            | E_Cmd::GrepResult
            | E_Cmd::CreateNewFile
            | E_Cmd::OpenFile(_)
            | E_Cmd::CloseFile
            | E_Cmd::CloseAllFile
            | E_Cmd::SaveFile
            | E_Cmd::StartEndRecordKey
            | E_Cmd::ExecRecordKey
            | E_Cmd::MouseMove(_, _)
            | E_Cmd::MouseDownLeftBox(_, _)
            | E_Cmd::MouseDragLeftBox(_, _)
            | E_Cmd::MouseModeSwitch
            | E_Cmd::Help
            | E_Cmd::OpenMenuFile
            | E_Cmd::OpenMenuConvert
            | E_Cmd::OpenMenuEdit
            | E_Cmd::OpenMenuSearch
            | E_Cmd::OpenMenuMacro
            | E_Cmd::BoxSelectMode
            | E_Cmd::CancelState
            | E_Cmd::ReOpenFile
            | E_Cmd::SwitchTabRight
            | E_Cmd::SwitchTabLeft
            | E_Cmd::InputComple
            | E_Cmd::Resize(_, _)
            | E_Cmd::SwitchDispScale
            | E_Cmd::SwitchDispRowNo
            | E_Cmd::Null => E_CmdConfig { is_edit: false, is_record: false },
        };
    }
}
