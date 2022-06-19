use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyWhen {
    All,
    Editor,
    MenuBar,
    FileBar,
    StatusBar,
    Prom,
    CtxMenu,
}

impl FromStr for KeyWhen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "allFocus" => Ok(KeyWhen::All),
            "editorFocus" => Ok(KeyWhen::Editor),
            "headerBarFocus" => Ok(KeyWhen::FileBar),
            "statusBarFocus" => Ok(KeyWhen::StatusBar),
            "promptFocus" => Ok(KeyWhen::Prom),
            _ => Err(()),
        }
    }
}

impl fmt::Display for KeyWhen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyWhen::Editor => write!(f, "editorFocus"),
            KeyWhen::MenuBar => write!(f, "menuBarFocus"),
            KeyWhen::FileBar => write!(f, "fileBarFocus"),
            KeyWhen::StatusBar => write!(f, "statusBarFocus"),
            KeyWhen::Prom => write!(f, "promptFocus"),
            KeyWhen::All => write!(f, "allFocus"),
            KeyWhen::CtxMenu => write!(f, "ctxMenuFocus"),
        }
    }
}
