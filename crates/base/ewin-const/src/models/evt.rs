use super::draw::*;

#[derive(Debug, Clone, Eq, PartialEq)]
// ActionType
pub enum ActType {
    Cancel, // Cancel process
    None,
    Exit,
    ExitMsg(String),
    Next, // Next Process
    Draw(DParts),
}
