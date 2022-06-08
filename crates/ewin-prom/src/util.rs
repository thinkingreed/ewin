use ewin_com::_cfg::key::keycmd::*;

pub fn is_select_proc(p_cmd: &P_Cmd) -> bool {
    match p_cmd {
        P_Cmd::MouseDragLeft(_, _) | P_Cmd::MouseDownLeft(_, _) | P_Cmd::CursorLeftSelect | P_Cmd::CursorRightSelect | P_Cmd::CursorRowHomeSelect | P_Cmd::CursorRowEndSelect => return true,
        _ => return false,
    }
}

pub fn is_edit_proc(p_cmd: &P_Cmd) -> bool {
    match p_cmd {
        P_Cmd::InsertStr(_) | P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Cut | P_Cmd::Undo | P_Cmd::Redo => return true,
        _ => return false,
    }
}
