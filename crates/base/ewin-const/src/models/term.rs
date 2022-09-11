use std::{hash::Hash, slice::Iter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Place {
    Tabs,
    Editor,
    MenuBar,
    FileBar,
    StatusBar,
    Prom,
    CtxMenu,
    Dialog,
    SideBar,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum CtxMenuPlace {
    Editor(CtxMenuPlaceEditorCond),
    FileBar,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// TermPlaceCondition
pub enum CtxMenuPlaceEditorCond {
    EditorRangeSelected,
    EditorRangeNonSelected,
    None,
}

impl CtxMenuPlace {
    pub fn iter() -> Iter<'static, CtxMenuPlace> {
        static TERM_PLACE: [CtxMenuPlace; 3] = [CtxMenuPlace::Editor(CtxMenuPlaceEditorCond::EditorRangeSelected), CtxMenuPlace::Editor(CtxMenuPlaceEditorCond::EditorRangeNonSelected), CtxMenuPlace::FileBar];
        TERM_PLACE.iter()
    }
}

impl CtxMenuPlace {
    pub fn from_str(place_str: &str, cond_str: &str) -> CtxMenuPlace {
        match place_str {
            "editor" => match cond_str {
                "range_selected" => CtxMenuPlace::Editor(CtxMenuPlaceEditorCond::EditorRangeSelected),
                "range_non_selected" => CtxMenuPlace::Editor(CtxMenuPlaceEditorCond::EditorRangeNonSelected),
                _ => CtxMenuPlace::None,
            },
            "file_bar" => CtxMenuPlace::FileBar,
            _ => CtxMenuPlace::None,
        }
    }
}
