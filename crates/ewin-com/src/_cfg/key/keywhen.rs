use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyWhen {
    AllFocus,
    InputFocus,
    EditorFocus,
    MenuBarFocus,
    FileBarFocus,
    StatusBarFocus,
    PromFocus,
    CtxMenuFocus,
}

impl FromStr for KeyWhen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "allFocus" => Ok(KeyWhen::AllFocus),
            "inputFocus" => Ok(KeyWhen::InputFocus),
            "editorFocus" => Ok(KeyWhen::EditorFocus),
            "headerBarFocus" => Ok(KeyWhen::FileBarFocus),
            "statusBarFocus" => Ok(KeyWhen::StatusBarFocus),
            "promptFocus" => Ok(KeyWhen::PromFocus),
            _ => Err(()),
        }
    }
}

impl fmt::Display for KeyWhen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyWhen::EditorFocus => write!(f, "editorFocus"),
            KeyWhen::MenuBarFocus => write!(f, "menuBarFocus"),
            KeyWhen::FileBarFocus => write!(f, "fileBarFocus"),
            KeyWhen::StatusBarFocus => write!(f, "statusBarFocus"),
            KeyWhen::PromFocus => write!(f, "promptFocus"),
            KeyWhen::InputFocus => write!(f, "inputFocus"),
            KeyWhen::AllFocus => write!(f, "allFocus"),
            KeyWhen::CtxMenuFocus => write!(f, "ctxMenuFocus"),
        }
    }
}
