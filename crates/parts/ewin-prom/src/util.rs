use ewin_key::key::cmd::*;

pub fn is_select_proc(cmd_type: &CmdType) -> bool {
    match cmd_type {
        CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseDownLeft(_, _) | CmdType::CursorLeftSelect | CmdType::CursorRightSelect | CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect => return true,
        _ => return false,
    }
}

pub fn is_edit_proc(cmd_type: &CmdType) -> bool {
    match cmd_type {
        CmdType::InsertStr(_) | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Cut | CmdType::Undo | CmdType::Redo => return true,
        _ => return false,
    }
}
