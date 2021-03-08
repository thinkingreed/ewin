use crate::{bar::headerbar::*, def::*, log::*, model::*};
use crossterm::event::{Event::*, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};

impl EvtAct {
    pub fn check_headerbar(hbar: &mut HeaderBar, editor: &mut Editor) -> EvtActType {
        Log::ep_s("　　　　　　　　check_headerbar");

        match editor.evt {
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                let (x, y) = (x as usize, y as usize);
                if y != hbar.disp_row_posi {
                    return EvtActType::Hold;
                } else if hbar.close_btn_area.0 <= x && x <= hbar.close_btn_area.1 {
                    editor.evt = CLOSE;
                    return EvtActType::Next;
                } else if hbar.help_btn_area.0 <= x && x <= hbar.help_btn_area.1 {
                    editor.evt = HELP;
                    return EvtActType::Hold;
                }
                return EvtActType::Hold;
            }
            _ => return EvtActType::Hold,
        }
    }
}
