use crossterm::cursor::position;
use ewin_core::{
    _cfg::key::keycmd::{Keybind, *},
    model::*,
};
use ewin_editor::model::Editor;
use ewin_term::{model::*, terminal::*};
use std::{env, io::stdout};
mod common;

#[test]
fn test_key_input() {
    common::setup();
    let mut term = Terminal::new();
    term.activate(&Args { filenm: "tests/file/key_input.txt".to_string(), ..Args::default() });

    eprintln!("111 {:?}", term.curt().editor.buf.text.to_string());

    // let key = Keybind::keycmd_to_keys(&KeyCmd::InsertLine);
    EvtAct::ctrl_editor(&mut term);
    let editor = Editor::new();

    editor.e_cmd = E_Cmd::InsertLine;

    // Editor::new()
    println!("");
}
