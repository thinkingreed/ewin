use super::keys::{Key, *};
use crate::{global::*, model::*, sel_range::*};
use ewin_cfg::model::modal::*;
use ewin_const::{
    def::*,
    models::{dialog::*, file::*, model::*, term::*, types::*},
    term::*,
};
use ewin_utils::files::file::FileOpenType;
use std::{cmp::Ordering, collections::BTreeSet};

impl Cmd {
    pub fn keys_to_cmd(keys: &Keys, keys_org_opt: Option<&Keys>, keywhen: Place) -> Cmd {
        let result = CMD_MAP.get().unwrap().get(&(*keys, Place::Tabs)).or_else(|| CMD_MAP.get().unwrap().get(&(*keys, keywhen)));

        let cmd = match result {
            Some(cmd) => cmd.clone(),
            None => Cmd::to_cmd(CmdType::Null),
        };
        if cmd.cmd_type != CmdType::Null {
            return cmd;
        }
        // common
        let cmd_type = match &keys {
            Keys::Resize(x, y) => CmdType::Resize(*x as usize, *y as usize),
            Keys::Raw(Key::Char(c)) => CmdType::InsertStr(c.to_string()),
            Keys::Shift(Key::Char(c)) => CmdType::InsertStr(c.to_ascii_uppercase().to_string()),
            Keys::MouseMove(y, x) => CmdType::MouseMove(*y as usize, *x as usize),
            Keys::MouseAltDownLeft(y, x) => CmdType::MouseDownLeftBox(*y as usize, *x as usize),
            Keys::MouseAltDragLeft(y, x) => CmdType::MouseDragLeftBox(*y as usize, *x as usize),
            Keys::MouseDownLeft(y, x) => CmdType::MouseDownLeft(*y as usize, *x as usize),
            Keys::MouseUpLeft(y, x) => CmdType::MouseUpLeft(*y as usize, *x as usize),
            Keys::MouseScrollUp => CmdType::MouseScrollUp,
            Keys::MouseScrollDown => CmdType::MouseScrollDown,

            Keys::MouseDragLeft(y, x) => {
                match keys_org_opt {
                    Some(Keys::MouseDragLeft(y_org, x_org)) | Some(Keys::MouseDownLeft(y_org, x_org)) => match y.cmp(y_org) {
                        Ordering::Less => CmdType::MouseDragLeftUp(*y as usize, *x as usize),
                        Ordering::Greater => CmdType::MouseDragLeftDown(*y as usize, *x as usize),
                        Ordering::Equal => {
                            if x == x_org {
                                CmdType::MouseDownLeft(*y as usize, *x as usize)
                            } else if x > x_org || x == &(get_term_size().0 as u16) {
                                CmdType::MouseDragLeftRight(*y as usize, *x as usize)
                            } else {
                                CmdType::MouseDragLeftLeft(*y as usize, *x as usize)
                            }
                        }
                    },
                    // dummy
                    _ => CmdType::MouseDragLeftDown(*y as usize, *x as usize),
                }
            }
            Keys::MouseDownRight(y, x) | Keys::MouseDragRight(y, x) => CmdType::CtxMenu(*y as usize, *x as usize),

            _ => CmdType::Null,
        };

        if cmd_type != CmdType::Null {
            return Cmd::to_cmd(cmd_type);
        }

        let cmd_type = match keywhen {
            Place::Editor => match &keys {
                Keys::Raw(Key::Tab) => CmdType::InsertStr(TAB_CHAR.to_string()),
                _ => CmdType::Unsupported,
            },
            /*
            KeyWhen::Prom => match &keys {
                Keys::Raw(Key::Tab) => CmdType::NextContent,
                Keys::Raw(Key::BackTab) => CmdType::BackContent,
                _ => CmdType::Unsupported,
            },
            */
            _ => CmdType::Unsupported,
        };
        return Cmd::to_cmd(cmd_type);
    }

    pub fn str_to_cmd(keycmd_str: &str) -> Cmd {
        return Cmd::to_cmd(Cmd::str_to_cmd_type(keycmd_str));
    }

    pub fn str_to_cmd_type(keycmd_str: &str) -> CmdType {
        let cmd_type = match keycmd_str {
            /*
             * Input
             */
            "undo" => CmdType::Undo,
            "redo" => CmdType::Redo,
            "delete_next_char" => CmdType::DelNextChar,
            "delete_prev_char" => CmdType::DelPrevChar,

            "cursor_left" => CmdType::CursorLeft,
            "cursor_right" => CmdType::CursorRight,
            "cursor_up" => CmdType::CursorUp,
            "cursor_down" => CmdType::CursorDown,
            "cursor_row_home" => CmdType::CursorRowHome,
            "cursor_row_end" => CmdType::CursorRowEnd,
            "cursor_left_select" => CmdType::CursorLeftSelect,
            "cursor_right_select" => CmdType::CursorRightSelect,
            "cursor_up_select" => CmdType::CursorUpSelect,
            "cursor_down_select" => CmdType::CursorDownSelect,
            "cursor_row_home_select" => CmdType::CursorRowHomeSelect,
            "cursor_row_end_select" => CmdType::CursorRowEndSelect,
            "cursor_file_home" => CmdType::CursorFileHome,
            "cursor_file_end" => CmdType::CursorFileEnd,
            "cursor_page_up" => CmdType::CursorPageUp,
            "cursor_page_down" => CmdType::CursorPageDown,

            "paste" => CmdType::InsertStr("".to_string()),
            "cut" => CmdType::Cut,
            "copy" => CmdType::Copy,
            "find_next" => CmdType::FindNext,
            "find_back" => CmdType::FindBack,
            "cancel_prompt" => CmdType::CancelProm,
            "confirm_prompt" => CmdType::Confirm,
            "next_content" => CmdType::NextContent,
            "back_content" => CmdType::BackContent,
            "find_case_sensitive" => CmdType::FindCaseSensitive,
            "find_regex" => CmdType::FindRegex,
            // file
            "close_file" => CmdType::CloseFileCurt(CloseFileType::Normal),
            "open_new_file" => CmdType::OpenNewFile,
            "close_all_file" => CmdType::CloseAllFile,
            "save_file" | "save_as" => CmdType::SaveFile(SaveFileType::Normal),
            // "save_as" => CmdType::SaveFile(SaveFileType::NewFile),
            "all_save_finish" => CmdType::SaveAllFinish,
            "switch_file_left" => CmdType::SwitchFile(Direction::Left),
            "switch_file_right" => CmdType::SwitchFile(Direction::Right),

            /*
             * editor
             */
            "all_select" => CmdType::AllSelect,
            "box_select_mode_start" => CmdType::BoxSelectModeStart,
            "insert_line" => CmdType::InsertRow,
            "format_json" | "json" => CmdType::Format(FileType::JSON),
            "format_xml" | "xml" => CmdType::Format(FileType::XML),
            "format_html" | "html" => CmdType::Format(FileType::HTML),

            // convert
            "to_full_width" => CmdType::Convert(ConvType::FullWidth),
            "to_half_width" => CmdType::Convert(ConvType::HalfWidth),
            "to_uppercase" => CmdType::Convert(ConvType::Uppercase),
            "to_lowercase" => CmdType::Convert(ConvType::Lowercase),
            "to_space" => CmdType::Convert(ConvType::Space),
            "to_tab" => CmdType::Convert(ConvType::Tab),

            // key macro
            "start_end_record_key" => CmdType::RecordKeyStartEnd,
            "exec_record_key" => CmdType::ExecRecordKey,
            // mouse
            "mouse_mode_switch" => CmdType::MouseModeSwitch,
            // menu
            "help" => CmdType::Help,
            "help_init_display_switch" => CmdType::HelpInitDisplaySwitch,
            "open_menu_file" => CmdType::OpenMenuFile,
            "open_menu_convert" => CmdType::OpenMenuConvert,
            "open_menu_edit" => CmdType::OpenMenuEdit,
            "open_menu_search" => CmdType::OpenMenuSearch,
            "open_menu_macro" => CmdType::OpenMenuMacro,
            // mode
            "cancel_editor_state" => CmdType::CancelEditorState,
            // context_menu
            "context_menu" => CmdType::CtxMenu(USIZE_UNDEFINED, USIZE_UNDEFINED),
            // input complement
            "input_complement" => CmdType::InputComple,
            /*
             * display
             */
            "scale" => CmdType::SwitchDispScale,
            "row_no" => CmdType::SwitchDispRowNo,
            "sidebar" => CmdType::SwitchDispSideBar,
            // prom
            "find_prompt" => CmdType::FindProm,
            "replace_prompt" => CmdType::ReplaceProm,
            "move_line_prompt" => CmdType::MoveRowProm,
            "grep_prompt" => CmdType::GrepProm,
            "encoding" | "encoding_prompt" => CmdType::EncodingProm,
            "open_file" | "open_file_prompt" => CmdType::openFileProm(OpenFileType::Normal),
            // window
            "window_split_vertical" | "left_and_right_split" => CmdType::WindowSplit(WindowSplitType::Vertical),
            "window_split_horizontal" | "top_and_bottom_split" => CmdType::WindowSplit(WindowSplitType::Horizontal),
            /*
             * Dialog
             */
            "about_app" => CmdType::DialogShow(DialogContType::AboutApp),
            // test
            "test" => CmdType::Test,
            _ => CmdType::Unsupported,
        };

        return cmd_type;
    }

    pub fn to_cmd(cmd_type: CmdType) -> Cmd {
        return match cmd_type {
            // edit
            CmdType::InsertStr(_) | CmdType::Cut | CmdType::InsertRow | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo | CmdType::InsertBox(_) | CmdType::DelBox(_) | CmdType::ReplaceExec(_, _, _) | CmdType::ReplaceTryExec(_, _) | CmdType::Format(_) | CmdType::Convert(_) => Cmd { cmd_type, place_vec: vec![Place::Editor, Place::Prom], config: CmdConfig { is_edit: true, is_record: true, is_recalc_scrl: true } },
            // key macro
            CmdType::RecordKeyStartEnd => Cmd { cmd_type, place_vec: vec![Place::Editor], ..Cmd::default() },
            CmdType::ExecRecordKey => Cmd { cmd_type, place_vec: vec![Place::Editor], config: CmdConfig { is_edit: true, is_recalc_scrl: true, ..CmdConfig::default() } },
            // Cursor move Editor and Prom
            CmdType::CursorLeft | CmdType::CursorRight | CmdType::CursorUp | CmdType::CursorDown | CmdType::CursorRowHome | CmdType::CursorRowEnd | CmdType::CursorLeftSelect | CmdType::CursorRightSelect | CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect => Cmd { cmd_type, place_vec: vec![Place::Editor, Place::Prom], config: CmdConfig { is_record: true, is_recalc_scrl: true, ..CmdConfig::default() } },
            // Cursor move Editor only
            CmdType::CursorUpSelect | CmdType::CursorDownSelect | CmdType::CursorFileHome | CmdType::CursorFileEnd | CmdType::CursorPageUp | CmdType::CursorPageDown => Cmd { cmd_type, place_vec: vec![Place::Editor], config: CmdConfig { is_record: true, is_recalc_scrl: true, ..CmdConfig::default() } },
            // select
            CmdType::Copy | CmdType::AllSelect => Cmd { cmd_type, place_vec: vec![Place::Editor, Place::Prom], config: CmdConfig { is_record: true, ..CmdConfig::default() } },
            // mouse
            CmdType::MouseDownLeft(_, _) | CmdType::MouseUpLeft(_, _) | CmdType::MouseMove(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseDownLeftBox(_, _) | CmdType::MouseDragLeftBox(_, _) | CmdType::MouseScrollUp | CmdType::MouseScrollDown => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, ..Cmd::default() },
            CmdType::MouseModeSwitch => Cmd { cmd_type, place_vec: vec![Place::Editor], ..Cmd::default() },

            CmdType::BoxSelectMode => Cmd { cmd_type, ..Cmd::default() },
            CmdType::BoxSelectModeStart => Cmd { cmd_type, ..Cmd::default() },
            /*
             * Tabs
             */
            CmdType::OpenNewFile | CmdType::OpenTgtFile(_) | CmdType::OpenGrepTgtFile(_) | CmdType::CloseFileCurt(_) | CmdType::CloseFileTgt(_) | CmdType::CloseAllFile | CmdType::CloseOtherThanThisTab(_) | CmdType::SwitchFile(_) | CmdType::SwapFile(_, _) | CmdType::ChangeFile(_) | CmdType::Help | CmdType::SaveAllFinish | CmdType::ClearTabState(_) | CmdType::ReOpenFile(_) => Cmd { cmd_type, place_vec: vec![Place::Tabs], ..Cmd::default() },
            // menu
            CmdType::HelpInitDisplaySwitch => Cmd { cmd_type, ..Cmd::default() },
            CmdType::OpenMenuFile | CmdType::OpenMenuConvert | CmdType::OpenMenuEdit | CmdType::OpenMenuSearch | CmdType::OpenMenuMacro => Cmd { cmd_type, ..Cmd::default() },
            /*
             * Editor
             */
            CmdType::Search(_, _) | CmdType::InputComple | CmdType::CancelEditorState | CmdType::SaveFile(_) | CmdType::MoveRow(_) => Cmd { cmd_type, place_vec: vec![Place::Editor], ..Cmd::default() },
            CmdType::WindowSplit(_) => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, place_vec: vec![Place::Editor] },

            // ContextMenu
            CmdType::CtxMenu(_, _) => Cmd { cmd_type, place_vec: vec![Place::Editor, Place::FileBar], ..Cmd::default() },
            // Prom
            CmdType::FindProm | CmdType::SaveForceProm | CmdType::SaveNewFileProm | CmdType::ReplaceProm | CmdType::MoveRowProm | CmdType::GrepProm | CmdType::GrepingProm(_) | CmdType::GrepResultProm | CmdType::EncodingProm | CmdType::openFileProm(_) | CmdType::WatchFileResultProm => Cmd { cmd_type, place_vec: vec![Place::Tabs], ..Cmd::default() },
            // CmdType::SaveAsFile => Cmd { cmd_type, place_vec: vec![Place::MenuBar], ..Cmd::default() },
            CmdType::FindNext | CmdType::FindBack => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, place_vec: vec![Place::Editor, Place::Prom] },
            CmdType::FindCaseSensitive | CmdType::FindRegex => Cmd { cmd_type, place_vec: vec![Place::Prom], ..Cmd::default() },

            CmdType::Confirm => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, place_vec: vec![Place::MenuBar, Place::Prom, Place::CtxMenu] },
            CmdType::CancelProm => Cmd { cmd_type, place_vec: vec![Place::Prom], ..Cmd::default() },
            CmdType::NextContent | CmdType::BackContent => Cmd { cmd_type, place_vec: vec![Place::Prom], ..Cmd::default() },
            /*
             * MenuBar
             */
            CmdType::MenuBarMenulist(_, _) => Cmd { cmd_type, ..Cmd::default() },
            // Display
            CmdType::SwitchDispScale | CmdType::SwitchDispRowNo => Cmd { cmd_type, place_vec: vec![Place::Editor], config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() } },
            CmdType::SwitchDispSideBar => Cmd { cmd_type, place_vec: vec![Place::SideBar], ..Cmd::default() },
            /*
             * Dialog
             */
            CmdType::DialogShow(_) => Cmd { cmd_type, place_vec: vec![Place::Dialog], ..Cmd::default() },
            /*
             * SideBar
             */
            CmdType::ChangeFileSideBar(_) => Cmd { cmd_type, place_vec: vec![Place::SideBar], ..Cmd::default() },
            // Other
            CmdType::Resize(_, _) => Cmd { cmd_type, config: CmdConfig { is_recalc_scrl: true, ..CmdConfig::default() }, ..Cmd::default() },
            CmdType::Null => Cmd { cmd_type, ..Cmd::default() },
            CmdType::Unsupported => Cmd { cmd_type, ..Cmd::default() },
            // Test
            CmdType::Test => Cmd { cmd_type, place_vec: vec![Place::Editor], ..Cmd::default() },
        };
    }

    pub fn get_cmd_str(cmd_type: &CmdType) -> String {
        let result = CMD_TYPE_MAP.get().unwrap().get(cmd_type);
        match result {
            Some(key) => key.to_string(),
            None => "".to_string(),
        }
    }
    pub fn cmd_to_keys(cmd_type: CmdType) -> Keys {
        *CMD_TYPE_MAP.get().unwrap().get(&cmd_type).unwrap()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Cmd {
    pub cmd_type: CmdType,
    pub config: CmdConfig,
    pub place_vec: Vec<Place>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum CmdType {
    /*
     * Prom
     */
    InsertStr(String),
    Cut,
    DelNextChar,
    DelPrevChar,
    Copy,
    Undo,
    Redo,
    InsertBox(Vec<(SelRange, String)>),
    DelBox(Vec<(SelRange, String)>),
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    CursorRowHome,
    CursorRowEnd,
    CursorLeftSelect,
    CursorRightSelect,
    CursorUpSelect,
    CursorDownSelect,
    CursorRowHomeSelect,
    CursorRowEndSelect,
    CursorFileHome,
    CursorFileEnd,
    AllSelect,
    CursorPageUp,
    CursorPageDown,
    FindNext,
    FindBack,
    MouseDownLeft(usize, usize),
    MouseUpLeft(usize, usize),
    MouseDragLeftUp(usize, usize),
    MouseDragLeftDown(usize, usize),
    MouseDragLeftLeft(usize, usize),
    MouseDragLeftRight(usize, usize),
    MouseDownLeftBox(usize, usize),
    MouseDragLeftBox(usize, usize),
    // MouseDragLeft(usize, usize),
    MouseMove(usize, usize),
    MouseScrollUp,
    MouseScrollDown,
    Resize(usize, usize),
    Confirm,
    CancelProm,
    NextContent,
    BackContent,
    FindCaseSensitive,
    FindRegex,
    Unsupported,
    // Editor
    BoxSelectModeStart,
    BoxSelectMode,
    InsertRow,
    Format(FileType),
    Convert(ConvType),
    RecordKeyStartEnd,
    ExecRecordKey,
    MouseModeSwitch,
    Help,
    HelpInitDisplaySwitch,
    OpenMenuFile,
    OpenMenuConvert,
    OpenMenuEdit,
    OpenMenuSearch,
    OpenMenuMacro,
    CancelEditorState,
    InputComple,
    Search(SearchType, String),
    MoveRow(usize),
    ReplaceTryExec(String, String),
    ReplaceExec(String, String, BTreeSet<usize>),

    // Window
    WindowSplit(WindowSplitType),

    // File
    CloseFileCurt(CloseFileType),
    CloseFileTgt(usize),
    ChangeFile(usize),
    OpenNewFile,
    OpenTgtFile(String),
    OpenGrepTgtFile(Search),
    CloseAllFile,
    CloseOtherThanThisTab(usize),
    SwitchFile(Direction),
    SwapFile(usize, usize),
    SaveAllFinish,
    ReOpenFile(FileOpenType),
    SaveFile(SaveFileType),

    // Tabs
    ClearTabState(bool),

    // Prom
    FindProm,
    ReplaceProm,
    MoveRowProm,
    SaveNewFileProm,
    SaveForceProm,
    GrepProm,
    GrepingProm(GrepInfo),
    GrepResultProm,
    EncodingProm,
    openFileProm(OpenFileType),
    WatchFileResultProm,
    // MenuBar
    MenuBarMenulist(usize, usize),
    SwitchDispScale,
    SwitchDispRowNo,
    SwitchDispSideBar,
    // CtxMenu
    CtxMenu(usize, usize),
    // Dialog
    DialogShow(DialogContType),
    // SideBar
    ChangeFileSideBar(String),

    Test,
    #[default]
    Null,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CmdConfig {
    pub is_edit: bool,
    pub is_record: bool,
    pub is_recalc_scrl: bool,
}
