use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyWhen {
    AllFocus,
    InputFocus,
    EditorFocus,
    HeaderBarFocus,
    StatusBarFocus,
    PromptFocus,
    CtxMenuFocus,
}

impl FromStr for KeyWhen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "allFocus" => Ok(KeyWhen::AllFocus),
            "inputFocus" => Ok(KeyWhen::InputFocus),
            "editorFocus" => Ok(KeyWhen::EditorFocus),
            "headerBarFocus" => Ok(KeyWhen::HeaderBarFocus),
            "statusBarFocus" => Ok(KeyWhen::StatusBarFocus),
            "promptFocus" => Ok(KeyWhen::PromptFocus),
            _ => Err(()),
        }
    }
}

impl fmt::Display for KeyWhen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyWhen::EditorFocus => write!(f, "editorFocus"),
            KeyWhen::HeaderBarFocus => write!(f, "headerBarFocus"),
            KeyWhen::StatusBarFocus => write!(f, "statusBarFocus"),
            KeyWhen::PromptFocus => write!(f, "promptFocus"),
            KeyWhen::InputFocus => write!(f, "inputFocus"),
            KeyWhen::AllFocus => write!(f, "allFocus"),
            KeyWhen::CtxMenuFocus => write!(f, "ctxMenuFocus"),
        }
    }
}
