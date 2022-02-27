use crate::{
    _cfg::model::{default::*, user::*},
    colors::*,
    def::*,
    log::*,
};
use std::env;

impl Cfg {
    /// Set user setting to internal setting
    pub fn set_user_setting(&mut self, cfg_user: CfgUser) {
        /*
         * general
         */
        /* general.lang */
        self.general.lang = match &cfg_user.general.lang {
            Some(s) if s == "ja_JP" => "ja_JP".to_string(),
            _ => "en_US".to_string(),
        };

        /* general.log */
        self.general.log.level = match &cfg_user.general.log.level {
            Some(s) if s == "debug" => "debug".to_string(),
            Some(s) if s == "error" => "error".to_string(),
            _ => "info".to_string(),
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
        if let Some(b) = cfg_user.general.editor.search.case_sens {
            self.general.editor.search.case_sens = b;
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
        /* general.editor.scrollbar.vertical */
        if let Some(u) = cfg_user.general.editor.scrollbar.vertical.width {
            self.general.editor.scrollbar.vertical.width = u;
        }
        /* general.editor.scrollbar.horizontal */
        if let Some(u) = cfg_user.general.editor.scrollbar.horizontal.height {
            self.general.editor.scrollbar.horizontal.height = u;
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
        // extension_when_saving_new_file
        if let Some(s) = cfg_user.general.editor.save.extension_when_saving_new_file {
            self.general.editor.save.extension_when_saving_new_file = s;
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
         * general.mouse
         */
        if let Some(b) = cfg_user.general.mouse.mouse_enable {
            self.general.mouse.mouse_enable = b;
        }
        /*
         * general.colors
         */
        /* theme */
        // highlight_theme_path
        if let Some(s) = cfg_user.general.colors.theme.highlight_theme_path {
            self.general.colors.theme.highlight_theme_path = Some(s);
        }
        // highlight_theme_background_enable
        if let Some(b) = cfg_user.general.colors.theme.highlight_theme_background_enable {
            self.general.colors.theme.highlight_theme_background_enable = Some(b);
        }
        // disable_highlight_ext
        if let Some(v) = cfg_user.general.colors.theme.disable_highlight_ext {
            self.general.colors.theme.disable_highlight_ext = v;
        }
        // disable_syntax_highlight_file_size
        if let Some(u) = cfg_user.general.colors.theme.disable_syntax_highlight_file_size {
            self.general.colors.theme.disable_syntax_highlight_file_size = u;
        }
        // default_color_theme
        if let Some(s) = cfg_user.general.colors.theme.default_color_theme {
            self.general.colors.theme.default_color_theme = s;
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
            TabType::HalfWidthBlank => " ".repeat(self.general.editor.tab.size),
        };

        self.general.editor.format.tab_type = TabType::from_str(&self.general.editor.format.indent_type);
        self.general.editor.format.indent = match self.general.editor.format.tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => " ".repeat(self.general.editor.format.indent_size),
        };
        self.general.editor.format.indent = match self.general.editor.format.tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => " ".repeat(self.general.editor.format.indent_size),
        };
    }
    pub fn convert_color_setting(&mut self) {
        Log::debug_key("Cfg.convert_color_setting");

        self.colors.system.btn.bg = Colors::hex2rgb(&self.colors.system.btn.background);
        self.colors.system.btn.fg = Colors::hex2rgb(&self.colors.system.btn.foreground);
        self.colors.system.state.bg = Colors::hex2rgb(&self.colors.system.state.background);
        self.colors.system.state.fg = Colors::hex2rgb(&self.colors.system.state.foreground);

        // self.colors.headerbar.fg = Colors::hex2rgb(&self.colors.headerbar.foreground);
        // self.colors.headerbar.bg = Colors::hex2rgb(&self.colors.headerbar.background);
        self.colors.headerbar.fg_tab_active = Colors::hex2rgb(&self.colors.headerbar.tab_active_foreground);
        self.colors.headerbar.bg_tab_active = Colors::hex2rgb(&self.colors.headerbar.tab_active_background);
        self.colors.headerbar.fg_tab_passive = Colors::hex2rgb(&self.colors.headerbar.tab_passive_foreground);
        self.colors.headerbar.bg_tab_passive = Colors::hex2rgb(&self.colors.headerbar.tab_passive_background);

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

        self.colors.editor.scrollbar.bg_vertical = Colors::hex2rgb(&self.colors.editor.scrollbar.vertical_background);
        self.colors.editor.scrollbar.bg_horizontal = Colors::hex2rgb(&self.colors.editor.scrollbar.horizontal_background);

        self.colors.msg.normal_fg = Colors::hex2rgb(&self.colors.msg.normal_foreground);
        self.colors.msg.highlight_fg = Colors::hex2rgb(&self.colors.msg.highlight_foreground);
        self.colors.msg.warning_fg = Colors::hex2rgb(&self.colors.msg.warning_foreground);
        self.colors.msg.err_fg = Colors::hex2rgb(&self.colors.msg.err_foreground);
        self.colors.statusbar.fg = Colors::hex2rgb(&self.colors.statusbar.foreground);

        self.colors.ctx_menu.fg_sel = Colors::hex2rgb(&self.colors.ctx_menu.select_foreground);
        self.colors.ctx_menu.fg_non_sel = Colors::hex2rgb(&self.colors.ctx_menu.non_select_foreground);
        self.colors.ctx_menu.bg_sel = Colors::hex2rgb(&self.colors.ctx_menu.select_background);
        self.colors.ctx_menu.bg_non_sel = Colors::hex2rgb(&self.colors.ctx_menu.non_select_background);

        self.colors.file.normal_fg = Colors::hex2rgb(&self.colors.file.normal_foreground);
        self.colors.file.directory_fg = Colors::hex2rgb(&self.colors.file.directory_foreground);
        self.colors.file.executable_fg = Colors::hex2rgb(&self.colors.file.executable_foreground);
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
        // foreground
        if let Some(s) = cfg_user_colors.system.state.foreground {
            self.system.state.foreground = s;
        }
        /*
         * HeaderBar
         */
        // tab_active_background
        if let Some(s) = cfg_user_colors.headerbar.tab_active_background {
            self.headerbar.tab_active_background = s;
        }
        // tab_active_foreground
        if let Some(s) = cfg_user_colors.headerbar.tab_active_foreground {
            self.headerbar.tab_active_foreground = s;
        }
        // tab_passive_background
        if let Some(s) = cfg_user_colors.headerbar.tab_passive_background {
            self.headerbar.tab_passive_background = s;
        }
        // tab_passive_foreground
        if let Some(s) = cfg_user_colors.headerbar.tab_passive_foreground {
            self.headerbar.tab_passive_foreground = s;
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
            self.editor.selection.background = s;
        }
        // foreground
        if let Some(s) = cfg_user_colors.editor.search.foreground {
            self.editor.selection.foreground = s;
        }
        /* control_char */
        // foreground
        if let Some(s) = cfg_user_colors.editor.control_char.foreground {
            self.editor.selection.foreground = s;
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
        /* Scrollbar */
        // horizontal_background
        if let Some(s) = cfg_user_colors.editor.scrollbar.horizontal_background {
            self.editor.scrollbar.horizontal_background = s;
        }
        // vertical_background
        if let Some(s) = cfg_user_colors.editor.scrollbar.vertical_background {
            self.editor.scrollbar.vertical_background = s;
        }
        /*
         * StatusBar
         */
        // foreground
        if let Some(s) = cfg_user_colors.statusbar.foreground {
            self.statusbar.foreground = s;
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
    }
}
