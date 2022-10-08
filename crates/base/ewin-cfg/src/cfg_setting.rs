use crate::{
    colors::*,
    log::*,
    model::{
        color::{default::*, user::*},
        general::{default::*, user::*},
    },
};
use ewin_const::{def::*, models::view::*};
use std::{cmp::max, env};

impl Cfg {
    /// Set user setting to internal setting
    pub fn set_user_setting(&mut self, cfg_user: CfgUser) {
        /*
         * general
         */
        /* general.lang */
        self.general.lang = match &cfg_user.general.lang {
            Some(s) if s == "ja_JP" => "ja_JP".to_string(),
            //  _ => "en_US".to_string(),
            _ => "ja_JP".to_string(),
        };
        /*
         * general.color_scheme
         */
        // default_color_theme
        self.general.color_scheme.default_color_theme = match &cfg_user.general.color_scheme.default_color_theme {
            Some(s) if s == "white" => "white".to_string(),
            Some(s) if s == "black" => "black".to_string(),
            _ => "white".to_string(),
        };
        /* general.log */
        match &cfg_user.general.log.level {
            Some(s) if s == "debug" => self.general.log.level = "debug".to_string(),
            Some(s) if s == "error" => self.general.log.level = "error".to_string(),
            _ => {}
        };
        /* general.font */
        if let Some(u) = cfg_user.general.font.ambiguous_width {
            self.general.font.ambiguous_width = Some(u);
        }
        /*
         * general.editor
         */
        /* general.editor.search */
        // case_sens
        if let Some(b) = cfg_user.general.editor.search.case_sensitive {
            self.general.editor.search.case_sensitive = b;
        }
        // regex
        if let Some(b) = cfg_user.general.editor.search.regex {
            self.general.editor.search.regex = b;
        }
        /* general.editor.tab */
        // size
        if let Some(u) = cfg_user.general.editor.tab.size {
            self.general.editor.tab.size = u;
        }
        // tab_input_type
        if let Some(s) = cfg_user.general.editor.tab.input_type {
            self.general.editor.tab.input_type = s;
        }
        /* general.editor.format */
        // indent_type
        if let Some(s) = cfg_user.general.editor.format.indent_type {
            self.general.editor.format.indent_type = s;
        }
        // indent_size
        if let Some(u) = cfg_user.general.editor.format.indent_size {
            self.general.editor.format.indent_size = u;
        }

        /* general.editor.cursor */
        if let Some(b) = cfg_user.general.editor.cursor.move_position_by_scrolling_enable {
            self.general.editor.cursor.move_position_by_scrolling_enable = b;
        }

        /* general.editor.column_char_width_gap_space */
        // character
        if let Some(c) = cfg_user.general.editor.column_char_width_gap_space.character {
            self.general.editor.column_char_width_gap_space.character = c;
        }
        // end_of_line_enable
        if let Some(b) = cfg_user.general.editor.column_char_width_gap_space.end_of_line_enable {
            self.general.editor.column_char_width_gap_space.end_of_line_enable = b;
        }
        /* general.editor.save */
        // use_string_first_line_for_file_name_of_new_file
        if let Some(b) = cfg_user.general.editor.save.use_string_first_line_for_file_name_of_new_file {
            self.general.editor.save.use_string_first_line_for_file_name_of_new_file = b;
        }
        // candidate_extension_when_saving_new_file
        if let Some(vec) = cfg_user.general.editor.save.candidate_extension_when_saving_new_file {
            self.general.editor.save.candidate_extension_when_saving_new_file = vec;
        }
        /* general.editor.word */
        // word_delimiter
        if let Some(s) = cfg_user.general.editor.word.word_delimiter {
            self.general.editor.word.word_delimiter = s;
        }
        /* general.editor.input_comple */
        // word_delimiter
        if let Some(s) = cfg_user.general.editor.input_comple.word_delimiter {
            self.general.editor.input_comple.word_delimiter = s;
        }
        // case_sens
        if let Some(b) = cfg_user.general.editor.input_comple.case_sensitive {
            self.general.editor.input_comple.case_sensitive = b;
        }
        /* general.editor.row_no */
        // is_enable
        if let Some(b) = cfg_user.general.editor.row_no.is_enable {
            self.general.editor.row_no.is_enable = b;
        }
        /* general.editor.scale */
        // is_enable
        if let Some(b) = cfg_user.general.editor.scale.is_enable {
            self.general.editor.scale.is_enable = b;
        }
        /* general.editor.scrollbar.vertical */
        if let Some(u) = cfg_user.general.editor.scrollbar.vertical.width {
            self.general.editor.scrollbar.vertical.width = u;
        }
        /* general.editor.scrollbar.horizontal */
        if let Some(u) = cfg_user.general.editor.scrollbar.horizontal.height {
            self.general.editor.scrollbar.horizontal.height = u;
        }
        /*
         * general.prompt
         */
        /* open_file */
        // directory_init_value
        self.general.prompt.open_file.directory_init_value = match &self.general.prompt.open_file.directory_init_value {
            s if s != &"current_directory".to_string() => self.general.prompt.open_file.directory_init_value.clone(),
            _ => env::current_dir().unwrap().to_string_lossy().to_string(),
        };
        /*
         * general.context_menu
         */
        if let Some(s) = cfg_user.general.context_menu.content {
            self.general.context_menu.content = s;
        }
        /*
         * general.menubar
         */
        if let Some(s) = cfg_user.general.menubar.content {
            self.general.menubar.content = s;
        }
        /*
         * general.mouse
         */
        if let Some(b) = cfg_user.general.mouse.mouse_enable {
            self.general.mouse.mouse_enable = b;
        }
        /*
         * general.view
         */
        // tab_characters_as_symbols
        if let Some(s) = cfg_user.general.view.tab_characters_as_symbols {
            self.general.view.tab_characters_as_symbols = s;
        } else {
            self.general.view.tab_characters_as_symbols = "^".to_string();
        }
        // full_width_space_characters_as_symbols
        if let Some(s) = cfg_user.general.view.full_width_space_characters_as_symbols {
            self.general.view.full_width_space_characters_as_symbols = s;
        } else {
            self.general.view.full_width_space_characters_as_symbols = "⬜".to_string();
        }

        /*
         * general.sidebar
         */
        // width
        if let Some(u) = cfg_user.general.sidebar.width {
            self.general.sidebar.width = u;
        }
        // scrollbar.vertical
        if let Some(u) = cfg_user.general.sidebar.scrollbar.vertical.width {
            self.general.sidebar.scrollbar.vertical.width = u;
        }
        // scrollbar.horizontal
        if let Some(u) = cfg_user.general.sidebar.scrollbar.horizontal.height {
            self.general.sidebar.scrollbar.horizontal.height = u;
        }
        // explorer.tree.indent
        if let Some(u) = cfg_user.general.sidebar.explorer.tree.indent {
            self.general.sidebar.explorer.tree.indent = u;
        }
        // explorer.quick_access.width
        if let Some(u) = cfg_user.general.sidebar.explorer.quick_access.width {
            self.general.sidebar.explorer.quick_access.width = u;
        }
        // explorer.quick_access.content
        if let Some(s) = cfg_user.general.sidebar.explorer.quick_access.content {
            self.general.sidebar.explorer.quick_access.content = s;
        }
        /*
         * general.activitybar
         */
        // width
        if let Some(u) = cfg_user.general.activitybar.width {
            // minimum is 2
            self.general.activitybar.width = max(u, 2);
        }
        // content
        if let Some(s) = cfg_user.general.activitybar.content {
            self.general.activitybar.content = s;
        }
        /*
         * general.tooltip
         */
        // width
        if let Some(u) = cfg_user.general.tooltip.hover_delay {
            self.general.tooltip.hover_delay = u;
        }

        /*
         * system
         */
        /* system.os */
        // change_output_encoding_utf8
        if let Some(b) = cfg_user.system.os.windows.change_output_encoding_utf8 {
            self.system.os.windows.change_output_encoding_utf8 = b;
        }
    }

    pub fn set_setting(&mut self) {
        self.general.editor.tab.tab_type = TabType::from_str(&self.general.editor.tab.input_type);
        self.general.editor.tab.tab = match self.general.editor.tab.tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => get_space(self.general.editor.tab.size),
        };

        self.general.editor.format.tab_type = TabType::from_str(&self.general.editor.format.indent_type);
        self.general.editor.format.indent = match self.general.editor.format.tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => get_space(self.general.editor.format.indent_size),
        };
        self.general.editor.format.indent = match self.general.editor.format.tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => get_space(self.general.editor.format.indent_size),
        };
    }
    pub fn convert_color_setting(&mut self) {
        Log::debug_key("Cfg.convert_color_setting");

        self.colors.system.btn.bg = Colors::hex2rgb(&self.colors.system.btn.background);
        self.colors.system.btn.fg = Colors::hex2rgb(&self.colors.system.btn.foreground);
        self.colors.system.state.bg = Colors::hex2rgb(&self.colors.system.state.background);
        self.colors.system.state.fg = Colors::hex2rgb(&self.colors.system.state.foreground);
        self.colors.system.scrollbar.bg_vertical = Colors::hex2rgb(&self.colors.system.scrollbar.vertical_background);
        self.colors.system.scrollbar.bg_horizontal = Colors::hex2rgb(&self.colors.system.scrollbar.horizontal_background);

        self.colors.menubar.fg_active = Colors::hex2rgb(&self.colors.menubar.active_foreground);
        self.colors.menubar.bg_active = Colors::hex2rgb(&self.colors.menubar.active_background);
        self.colors.menubar.fg_passive = Colors::hex2rgb(&self.colors.menubar.passive_foreground);
        self.colors.menubar.bg_passive = Colors::hex2rgb(&self.colors.menubar.passive_background);
        self.colors.menubar.bg_default = Colors::hex2rgb(&self.colors.menubar.default_background);

        self.colors.filebar.fg_active = Colors::hex2rgb(&self.colors.filebar.active_foreground);
        self.colors.filebar.bg_active = Colors::hex2rgb(&self.colors.filebar.active_background);
        self.colors.filebar.fg_passive = Colors::hex2rgb(&self.colors.filebar.passive_foreground);
        self.colors.filebar.bg_passive = Colors::hex2rgb(&self.colors.filebar.passive_background);
        self.colors.filebar.bg_default = Colors::hex2rgb(&self.colors.filebar.default_background);

        self.colors.editor.fg = Colors::hex2rgb(&self.colors.editor.foreground);
        self.colors.editor.bg = Colors::hex2rgb(&self.colors.editor.background);

        self.colors.editor.line_number.active_bg = Colors::hex2rgb(&self.colors.editor.line_number.active_background);
        self.colors.editor.line_number.active_fg = Colors::hex2rgb(&self.colors.editor.line_number.active_foreground);
        self.colors.editor.line_number.passive_bg = Colors::hex2rgb(&self.colors.editor.line_number.passive_background);
        self.colors.editor.line_number.passive_fg = Colors::hex2rgb(&self.colors.editor.line_number.passive_foreground);

        self.colors.editor.selection.bg = Colors::hex2rgb(&self.colors.editor.selection.background);
        self.colors.editor.selection.fg = Colors::hex2rgb(&self.colors.editor.selection.foreground);
        self.colors.editor.search.bg = Colors::hex2rgb(&self.colors.editor.search.background);
        self.colors.editor.search.fg = Colors::hex2rgb(&self.colors.editor.search.foreground);
        self.colors.editor.control_char.fg = Colors::hex2rgb(&self.colors.editor.control_char.foreground);
        self.colors.editor.column_char_width_gap_space.fg = Colors::hex2rgb(&self.colors.editor.column_char_width_gap_space.foreground);
        self.colors.editor.column_char_width_gap_space.bg = Colors::hex2rgb(&self.colors.editor.column_char_width_gap_space.background);

        self.colors.editor.scale.bg = Colors::hex2rgb(&self.colors.editor.scale.background);
        self.colors.editor.scale.fg = Colors::hex2rgb(&self.colors.editor.scale.foreground);

        self.colors.editor.window.split_line.bg = Colors::hex2rgb(&self.colors.editor.window.split_line.background);

        // msg
        self.colors.msg.normal_fg = Colors::hex2rgb(&self.colors.msg.normal_foreground);
        self.colors.msg.highlight_fg = Colors::hex2rgb(&self.colors.msg.highlight_foreground);
        self.colors.msg.warning_fg = Colors::hex2rgb(&self.colors.msg.warning_foreground);
        self.colors.msg.err_fg = Colors::hex2rgb(&self.colors.msg.err_foreground);
        self.colors.msg.bg = Colors::hex2rgb(&self.colors.msg.background);

        // statusbar
        self.colors.statusbar.fg = Colors::hex2rgb(&self.colors.statusbar.foreground);
        self.colors.statusbar.bg = Colors::hex2rgb(&self.colors.statusbar.background);

        // ctx_menu
        self.colors.ctx_menu.fg_sel = Colors::hex2rgb(&self.colors.ctx_menu.select_foreground);
        self.colors.ctx_menu.fg_non_sel = Colors::hex2rgb(&self.colors.ctx_menu.non_select_foreground);
        self.colors.ctx_menu.bg_sel = Colors::hex2rgb(&self.colors.ctx_menu.select_background);
        self.colors.ctx_menu.bg_non_sel = Colors::hex2rgb(&self.colors.ctx_menu.non_select_background);
        // dialog
        self.colors.dialog.fg_default = Colors::hex2rgb(&self.colors.dialog.default_foreground);
        self.colors.dialog.bg_default = Colors::hex2rgb(&self.colors.dialog.default_background);
        self.colors.dialog.fg_header = Colors::hex2rgb(&self.colors.dialog.header_foreground);
        self.colors.dialog.bg_header = Colors::hex2rgb(&self.colors.dialog.header_background);
        self.colors.dialog.bg_sel = Colors::hex2rgb(&self.colors.dialog.select_background);

        // file
        self.colors.file.normal_fg = Colors::hex2rgb(&self.colors.file.normal_foreground);
        self.colors.file.directory_fg = Colors::hex2rgb(&self.colors.file.directory_foreground);
        self.colors.file.executable_fg = Colors::hex2rgb(&self.colors.file.executable_foreground);

        // sidebar
        self.colors.sidebar.fg = Colors::hex2rgb(&self.colors.sidebar.foreground);
        self.colors.sidebar.bg = Colors::hex2rgb(&self.colors.sidebar.background);
        self.colors.sidebar.bg_header = Colors::hex2rgb(&self.colors.sidebar.header_background);
        self.colors.sidebar.bg_open_file = Colors::hex2rgb(&self.colors.sidebar.open_file_background);

        // activitybar
        self.colors.activitybar.bg_default = Colors::hex2rgb(&self.colors.activitybar.default_background);
        self.colors.activitybar.bg_select = Colors::hex2rgb(&self.colors.activitybar.select_background);

        // tooltip
        self.colors.tooltip.bg = Colors::hex2rgb(&self.colors.tooltip.background);
        self.colors.tooltip.fg = Colors::hex2rgb(&self.colors.tooltip.foreground);
    }
}

impl CfgColors {
    /// Set user setting to internal setting
    pub fn set_user_setting(&mut self, cfg_user_colors: CfgUserColors) {
        /*
         * System
         */
        /* SystemBtn */
        // background
        if let Some(s) = cfg_user_colors.system.btn.background {
            self.system.btn.background = s;
        }
        // foreground
        if let Some(s) = cfg_user_colors.system.btn.foreground {
            self.system.btn.foreground = s;
        }
        /* SystemState */
        // background
        if let Some(s) = cfg_user_colors.system.state.background {
            self.system.state.background = s;
        }
        /* Scrollbar */
        // horizontal_background
        if let Some(s) = cfg_user_colors.system.scrollbar.horizontal_background {
            self.system.scrollbar.horizontal_background = s;
        }
        // vertical_background
        if let Some(s) = cfg_user_colors.system.scrollbar.vertical_background {
            self.system.scrollbar.vertical_background = s;
        }

        // foreground
        if let Some(s) = cfg_user_colors.system.state.foreground {
            self.system.state.foreground = s;
        }

        /* theme */
        // highlight_theme_path
        if let Some(s) = cfg_user_colors.theme.highlight_theme_path {
            self.theme.highlight_theme_path = Some(s);
        }
        // highlight_theme_background_enable
        if let Some(b) = cfg_user_colors.theme.highlight_theme_background_enable {
            self.theme.highlight_theme_background_enable = Some(b);
        }
        // disable_highlight_ext
        if let Some(v) = cfg_user_colors.theme.disable_highlight_ext {
            self.theme.disable_highlight_ext = v;
        }
        // disable_syntax_highlight_file_size
        if let Some(u) = cfg_user_colors.theme.disable_syntax_highlight_file_size {
            self.theme.disable_syntax_highlight_file_size = u;
        }

        /*
         * MenuBar
         */
        // menu_active_background
        if let Some(s) = cfg_user_colors.menubar.active_background {
            self.menubar.active_background = s;
        }
        // menu_active_foreground
        if let Some(s) = cfg_user_colors.menubar.active_foreground {
            self.menubar.active_foreground = s;
        }
        // menu_passive_background
        if let Some(s) = cfg_user_colors.menubar.passive_background {
            self.menubar.passive_background = s;
        }
        // menu_passive_foreground
        if let Some(s) = cfg_user_colors.menubar.passive_foreground {
            self.menubar.passive_foreground = s;
        }
        // menu_default_background
        if let Some(s) = cfg_user_colors.menubar.default_background {
            self.menubar.default_background = s;
        }
        /*
         * FileBar
         */
        // tab_active_background
        if let Some(s) = cfg_user_colors.filebar.active_background {
            self.filebar.active_background = s;
        }
        // tab_active_foreground
        if let Some(s) = cfg_user_colors.filebar.active_foreground {
            self.filebar.active_foreground = s;
        }
        // tab_passive_background
        if let Some(s) = cfg_user_colors.filebar.passive_background {
            self.filebar.passive_background = s;
        }
        // tab_passive_foreground
        if let Some(s) = cfg_user_colors.filebar.passive_foreground {
            self.filebar.passive_foreground = s;
        }
        // default_background
        if let Some(s) = cfg_user_colors.filebar.default_background {
            self.filebar.default_background = s;
        }
        /*
         * Editor
         */
        // background
        if let Some(s) = cfg_user_colors.editor.background {
            self.editor.background = s;
        }
        if let Some(s) = cfg_user_colors.editor.foreground {
            self.editor.foreground = s;
        }
        /* LineNumber */
        // active_background
        if let Some(s) = cfg_user_colors.editor.line_number.active_background {
            self.editor.line_number.active_background = s;
        }
        // active_foreground
        if let Some(s) = cfg_user_colors.editor.line_number.active_foreground {
            self.editor.line_number.active_foreground = s;
        }
        // passive_background
        if let Some(s) = cfg_user_colors.editor.line_number.passive_background {
            self.editor.line_number.passive_background = s;
        }
        // passive_foreground
        if let Some(passive_foreground) = cfg_user_colors.editor.line_number.passive_foreground {
            self.editor.line_number.passive_foreground = passive_foreground;
        }
        /* Selection */
        // background
        if let Some(s) = cfg_user_colors.editor.selection.background {
            self.editor.selection.background = s;
        }
        // foreground
        if let Some(s) = cfg_user_colors.editor.selection.foreground {
            self.editor.selection.foreground = s;
        }
        /* search */
        // background
        if let Some(s) = cfg_user_colors.editor.search.background {
            self.editor.search.background = s;
        }
        // foreground
        if let Some(s) = cfg_user_colors.editor.search.foreground {
            self.editor.search.foreground = s;
        }
        /* control_char */
        // foreground
        if let Some(s) = cfg_user_colors.editor.control_char.foreground {
            self.editor.control_char.foreground = s;
        }
        /* column_char_width_gap */
        // background
        if let Some(s) = cfg_user_colors.editor.column_char_width_gap_space.background {
            self.editor.column_char_width_gap_space.background = s;
        }
        // foreground
        if let Some(s) = cfg_user_colors.editor.column_char_width_gap_space.foreground {
            self.editor.column_char_width_gap_space.foreground = s;
        }
        /* Scale */
        // background
        if let Some(s) = cfg_user_colors.editor.scale.background {
            self.editor.scale.background = s;
        }
        // foreground
        if let Some(s) = cfg_user_colors.editor.scale.foreground {
            self.editor.scale.foreground = s;
        }
        /* Window */
        // split_line
        // background
        if let Some(window) = cfg_user_colors.editor.window {
            if let Some(split_line) = window.split_line {
                if let Some(s) = split_line.background {
                    self.editor.window.split_line.background = s;
                }
            }
        }
        /*
         * StatusBar
         */
        // foreground
        if let Some(s) = cfg_user_colors.statusbar.foreground {
            self.statusbar.foreground = s;
        }
        // background
        if let Some(s) = cfg_user_colors.statusbar.background {
            self.statusbar.background = s;
        }
        /*
         * Ctx_menu
         */
        // non_select_background
        if let Some(s) = cfg_user_colors.ctx_menu.non_select_background {
            self.ctx_menu.non_select_background = s;
        }
        // non_select_foreground
        if let Some(s) = cfg_user_colors.ctx_menu.non_select_foreground {
            self.ctx_menu.non_select_foreground = s;
        }
        // select_background
        if let Some(s) = cfg_user_colors.ctx_menu.select_background {
            self.ctx_menu.select_background = s;
        }
        // select_foreground
        if let Some(s) = cfg_user_colors.ctx_menu.select_foreground {
            self.ctx_menu.select_foreground = s;
        }
        /*
         * Msg
         */
        // select_foreground
        if let Some(s) = cfg_user_colors.msg.normal_foreground {
            self.msg.normal_foreground = s;
        }
        // highlight_foreground
        if let Some(s) = cfg_user_colors.msg.highlight_foreground {
            self.msg.highlight_foreground = s;
        }
        // warning_foreground
        if let Some(s) = cfg_user_colors.msg.warning_foreground {
            self.msg.warning_foreground = s;
        }
        // err_foreground
        if let Some(s) = cfg_user_colors.msg.err_foreground {
            self.msg.err_foreground = s;
        }
        // background
        if let Some(s) = cfg_user_colors.msg.background {
            self.msg.background = s;
        }
        /*
         * file
         */
        // normal_foreground
        if let Some(s) = cfg_user_colors.file.normal_foreground {
            self.file.normal_foreground = s;
        }
        // directory_foreground
        if let Some(s) = cfg_user_colors.file.directory_foreground {
            self.file.directory_foreground = s;
        }
        // executable_foreground
        if let Some(s) = cfg_user_colors.file.executable_foreground {
            self.file.executable_foreground = s;
        }
        /*
         * Dialog
         */
        // default_foreground
        if let Some(s) = cfg_user_colors.dialog.default_foreground {
            self.dialog.default_foreground = s;
        }
        // default_background
        if let Some(s) = cfg_user_colors.dialog.default_background {
            self.dialog.default_background = s;
        }
        // header_foreground
        if let Some(s) = cfg_user_colors.dialog.header_foreground {
            self.dialog.header_foreground = s;
        }
        // header_background
        if let Some(s) = cfg_user_colors.dialog.header_background {
            self.dialog.header_background = s;
        }
        // select_background
        if let Some(s) = cfg_user_colors.dialog.select_background {
            self.dialog.select_background = s;
        }
        /*
         * SideBar
         */
        // foreground
        if let Some(s) = cfg_user_colors.sdiebar.foreground {
            self.sidebar.foreground = s;
        }
        // background
        if let Some(s) = cfg_user_colors.sdiebar.background {
            self.sidebar.background = s;
        }
        // header_background
        if let Some(s) = cfg_user_colors.sdiebar.header_background {
            self.sidebar.header_background = s;
        }
        // open_file_background
        if let Some(s) = cfg_user_colors.sdiebar.open_file_background {
            self.sidebar.open_file_background = s;
        }

        /*
         * ActivityBar
         */
        // default_background
        if let Some(s) = cfg_user_colors.activitybar.background_default {
            self.activitybar.default_background = s;
        }
        // select_background
        if let Some(s) = cfg_user_colors.activitybar.background_select {
            self.activitybar.select_background = s;
        }
        /*
         * ToolTip
         */
        // background
        if let Some(s) = cfg_user_colors.tooltip.background {
            self.tooltip.background = s;
        }
        // foreground
        if let Some(s) = cfg_user_colors.tooltip.foreground {
            self.tooltip.foreground = s;
        }
    }
}
