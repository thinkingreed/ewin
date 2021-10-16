use ewin_com::model::*;
use ewin_term::terminal::*;

mod common;

#[test]
fn test_key_input() {
    common::setup();
    let mut term = Terminal::new();
    term.activate(&Args { filenm: "tests/file/key_input.txt".to_string(), ..Args::default() });

    eprintln!("111 {:?}", term.curt().editor.buf.text.to_string());

    /*
    EvtAct::ctrl_editor(&mut term);
    let mut editor = Editor::new();

    editor.e_cmd = E_Cmd::InsertLine;

    // Editor::new()
    println!("");
     */
}
