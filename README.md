# ewin

**_Simple editor for Window(GUI) users._**

**_No need to remember commands_**

It provides basic features as a minimal text editor:

- Open/Save text files
- Create new text files and empty text buffer on memory
- Edit a text (put/delete characters, insert/delete lines, ...)
- Support editing UTF-8 characters
- Resizing terminal window supported. Screen size is responsible
- Key binding support
- Mouse support
- Tab support

## Installation

### On Ubuntu

_... and other Debian-based Linux distributions_

Download the latest .deb package from the [release page](https://github.com/thinkingreed/ewin/releases) and install it via:

### On CentOS

Download the latest .rpm package from the [release page](https://github.com/thinkingreed/ewin/releases) and install it via:

```
rpm -ivh ewin_0.0.0.x86_64.rpm
```

### Via Snap

Please install Snap Store or Command

```
$ sudo snap install --edge ewin
```

## Usage

### CLI

Installing package introduces `ewin` command in your system.

```sh
$ ewin file         # Open files to edit
```

Please see `ewin --help` for command usage.

## Operation

ewin is a mode-less text editor. Like other famous mode-less text editors such as Nano, Emacs, you can edit text in terminal window using a keyboard.
And several keys with Ctrl or Alt modifiers are mapped to various features.

- **Operations**

| Mapping        | Motion                                                                                                              | Key recording |
| -------------- | ------------------------------------------------------------------------------------------------------------------- | ------------- |
| `Ctrl` + `w`   | Quit.                                                                                                               | ―             |
| `Ctrl` + `s`   | Save current buffer to file.                                                                                        | ―             |
| `F1`           | Key binding display at the bottom of the screen.                                                                    | ―             |
| `Ctrl` + `f`   | Enter the characters to incremental search.Search target is open file.                                              | ―             |
| `Ctrl` + `g`   | Grep. Enter the characters you want to search for. The search target is the UTF-8 file of the entered file pattern. | ―             |
|                | Linux・WSL                                                                                                          | ―             |
|                | Command to use : grep -rHnI search_str --include=search_filenm search_folder                                        |               |
|                | -r:Subfolder search,-H:File name display,-n:Line number display,-I: Binary file not applicable                      |               |
|                | Windows 　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　                              | ―             |
|                | Command to use : dir -recurse search_folder -Exclude Binary file... \| Select-String search_str                     |               |
| `Shift` + `F1` | Key record start or stop.Recording key operation.                                                                   | ―             |
| `Shift` + `F2` | Execution of the recorded key.                                                                                      | ―             |
| `Ctrl` + `o`   | Open new file.                                                                                                      | ―             |
| `Ctrl` + `n`   | Create new tab. Another operation is to double-click the header.                                                    | ―             |
|                | In case of Windows, it will not be recognized unless you double-click slowly                                        |               |
| `Ctrl` + `q`   | Change next tab.                                                                                                    | ―             |
| `Ctrl` + `e`   | Specify the character code and reload. Or set the character code line feed code and BOM.                            | ―             |
| `F12`          | Mouse capture changes.Used for clipboard access via terminal app                                                    | ―             |
|                | when connecting to a remote terminal                                                                                | ―             |

- **Moving cursor**

| Mapping                   | Motion                         | Key recording |
| ------------------------- | ------------------------------ | ------------- |
| `↑` or `Mouse ScrollUp`   | Move cursor up.                | target        |
| `↓` or `Mouse ScrollDown` | Move cursor down.              | target        |
| `→`                       | Move cursor right.             | target        |
| `←`                       | Move cursor left.              | target        |
| `HOME`                    | Move cursor to head of line.   | target        |
| `END`                     | Move cursor to end of line.    | target        |
| `PAGE DOWN`               | Next page.                     | target        |
| `PAGE UP`                 | Previous page.                 | target        |
| `Ctrl` + `HOME`           | Move cursor to first of line.  | target        |
| `Ctrl` + `END`            | Move cursor to last of line.   | target        |
| `Ctrl` + `l`              | Move cursor to specified line. | ―             |

- **Edit text**
  | Mapping | Motion | Key recording |
  |--------------|---------------------------------------------------------------------------------|---------------|
  | `Enter` | Insert new line 　　　　　　　　　　　　　　　　　 |target 　　 　 |
  | `BACKSPACE` | Delete character 　　　　　　　　　　　　　　　　　　|target |
  | `DELETE` | Delete next character |target |
  | `Tab` | Insert new tab |target |
  | `Ctrl` + `x` | Select range cut. |target |
  | `Ctrl` + `c` | Select range copy.If you want to copy to the clipboard when operating remotely, use F12 to switch the mouse operation. |target |
  | `Ctrl` + `v` | Paste the copied characters. |target |
  | `Ctrl` + `r` | Replace character. |― |
  | `Ctrl` + `z` | Undo.Undo the last edit. |― |
  | `Ctrl` + `y` | Redo.Make the last update again. |― |

* **Select text**

| Mapping                    | Motion                                                                         | Key recording |
| -------------------------- | ------------------------------------------------------------------------------ | ------------- |
| `Shift` + `↑`              | Select from the beginning of the current line and the end of the line above    | target        |
| `Shift` + `↓`              | Select from the end of the current line and the beginning of the line below    | target        |
| `Shift` + `→`              | Select the next character.　　　　　　　　　　　　　　　　　　　　　　　　　　 | target        |
| `Shift` + `←`              | Select the previous character.                                                 | target        |
| `Shift` + `HOME`           | Select the head of line.                                                       | target        |
| `Shift` + `END`            | Select the end of line.                                                        | target        |
| `Ctrl` + `a`               | Select all.                                                                    | target        |
| `F3`                       | Search for characters below. 　　　　　                                        | target        |
| `Shift` + `F4`             | Search for above characters below.　　　　　                                   | target        |
| `Mouse Left.Down` + `Drag` | Select a range.                                                                | ―             |
| `Mouse Double click`       | Select a range.Delimiter is `` ! "\#$%&()*+-',./:;<=>?@[]^`{|}~ ``             | ―             |
| `Mouse Triple click`       | Select a line.                                                                 | ―             |

## Operation restrictions

| motion          | Mapping           | environment | Contents                                                                                                                         |
| --------------- | ----------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `Key record`    | `Ctrl` + `F1`     | WSL         | keybindings.command."copy" and "paste" needs to be changed to something other than Ctrl+c, Ctrl+v. Ex)Ctrl+Shift+c, Ctrl+Shift+v |
| `Save` 　       | `Ctrl` + `s`      | All         | Forcibly convert CR + LF of line feed code to LF                                                                                 |
| `Copy`・`Paste` | `Ctrl` + `c`・`v` | WSL         | Need path to powershell.exe. Try \$PSHOME at PowerShell terminal                                                                 |

## Setting file

Please edit the contents of setting.toml(initial setting) below and put it in this location.

### On Linux

\$HOME/.config/ewin/setting.toml

### On Windows

%USERPROFILE%\AppData\Roaming\ewin\setting.toml

```
#################################################################
[general.editor.search]
case_sens = false

# If theme is set, theme color has the highest priority
[colors.theme]
# theme_path = "tmTheme.tmTheme"
# theme_background_enable = true

[colors.editor]
background = "#000000"
foreground = "#ffffff"

[colors.editor.line_number]
background = "#000000"
foreground = "#6e6e6e"

[colors.editor.selection]
background = "#dd4814"
foreground = "#000000"

[colors.editor.search]
background = "#dd4814"
foreground = "#000000"

[colors.editor.control_char]
foreground = "#6e6e6e"

[colors.status_bar]
foreground = "#87411f"

[colors.msg]
normal_foreground = "#ffffff"
highlight_foreground = "#00c800"
warning_foreground = "#ffa500"
err_foreground = "#ff0000"
#################################################################
```

## Key binding file

Please edit the contents of keybind.json5(initial setting) below and put it in this location.

### On Linux

\$HOME/.config/ewin/keybind.json5

### On Windows

%USERPROFILE%\AppData\Roaming\ewin\keybind.json5

```
#################################################################
[
  /* key
     // Raw
         left, right, up, down, home, end, enter, backspace, delete, pageup, pagedown, esc, F1～F12
     // Ctrl
         ctrl+(left, right, up, down, home, end, enter, backspace, delete, pageup, pagedown, tab, esc, a～z, F1～F12)
     // Shit
         shit+(left, right, up, down, home, end, enter, backspace, delete, pageup, pagedown, tab, esc, a～z, F1～F12)
     // Alt
         alt+(left, right, up, down, home, end, enter, backspace, delete, pageup, pagedown, tab, esc, a～z, F1～F12)
  /* cmd
     // cursor move
         cursorLeft, cursorRight, cursorUp, cursorDown, cursorRowHome, cursorRowEnd, cursorFileHome, cursorFileEnd, cursorPageUp, cursorPageDown
     // select
         cursorLeftSelect, cursorRightSelect, cursorUpSelect, cursorDownSelect, cursorRowHomeSelect, cursorRowEndSelect, allSelect
     // edit
         insertLine, deletePrevChar, deleteNextChar, cutSelect, copySelect, paste, undo, redo
     // find
         find, findNext, findBack, replace, moveLine, grep
     // file
         newTab, nextTab, openFile, encoding, closeFile, closeAllFile, saveFile
     // key record・mouse
         startEndRecordKey, execRecordKey, mouseOpeSwitch
     // menu
         help, openMenu, openMenuFile, openMenuConvert, openMenuEdit, openMenuSelect
     // prompt
         escPrompt, confirmPrompt, findCaseSensitive, findRegex
  /* when
     allFocus, inputFocus, editorFocus, promptFocus
  /****************
   * editorFocus
   ****************/

  /* cursor move */
  { key: 'left',       cmd: 'cursorLeft',          when: 'inputFocus'},
  { key: 'right',      cmd: 'cursorRight',         when: 'inputFocus'},
  { key: 'down',       cmd: 'cursorDown',          when: 'inputFocus'},
  { key: 'up',         cmd: 'cursorUp',            when: 'inputFocus'},
  { key: 'home',       cmd: 'cursorRowHome',       when: 'inputFocus'},
  { key: 'end',        cmd: 'cursorRowEnd',        when: 'inputFocus'},
  { key: 'ctrl+home',  cmd: 'cursorFileHome',      when: 'inputFocus'},
  { key: 'ctrl+end',   cmd: 'cursorFileEnd',       when: 'inputFocus'},
  { key: 'pagedown',   cmd: 'cursorPageDown',      when: 'inputFocus'},
  { key: 'pageup',     cmd: 'cursorPageUp',        when: 'inputFocus'},

  /* select */
  { key: 'shift+left', cmd: 'cursorLeftSelect',    when: 'inputFocus'},
  { key: 'shift+right',cmd: 'cursorRightSelect',   when: 'inputFocus'},
  { key: 'shift+down', cmd: 'cursorDownSelect',    when: 'inputFocus'},
  { key: 'shift+up',   cmd: 'cursorUpSelect',      when: 'inputFocus'},
  { key: 'shift+home', cmd: 'cursorRowHomeSelect', when: 'inputFocus'},
  { key: 'shift+end',  cmd: 'cursorRowEndSelect',  when: 'inputFocus'},
  { key: 'ctrl+a',     cmd: 'allSelect',           when: 'inputFocus'},

  /* edit */
  { key: 'enter',      cmd: 'insertLine',          when: 'editorFocus'},
  { key: 'backspace',  cmd: 'deletePrevChar',      when: 'inputFocus'},
  { key: 'delete',     cmd: 'deleteNextChar',      when: 'inputFocus'},
  { key: 'ctrl+x',     cmd: 'cutSelect',           when: 'inputFocus'},
  { key: 'ctrl+c',     cmd: 'copySelect',          when: 'inputFocus'},
  { key: 'ctrl+v',     cmd: 'paste',               when: 'inputFocus'},
  { key: 'ctrl+z',     cmd: 'undo',                when: 'inputFocus'},
  { key: 'ctrl+y',     cmd: 'redo',                when: 'inputFocus'},

  /* find */
  { key: 'ctrl+f',     cmd: 'find',                when: 'editorFocus'},
  { key: 'F3',         cmd: 'findNext',            when: 'editorFocus'},
  { key: 'shift+F4',   cmd: 'findBack',            when: 'editorFocus'},
  { key: 'ctrl+r',     cmd: 'replace',             when: 'editorFocus'},
  { key: 'ctrl+l',     cmd: 'moveLine',            when: 'editorFocus'},
  { key: 'ctrl+g',     cmd: 'grep',                when: 'editorFocus'},

  /* file */
  { key: 'ctrl+w',     cmd: 'closeFile',           when: 'allFocus'},
  { key: 'ctrl+n',     cmd: 'newTab',              when: 'editorFocus'},
  { key: 'ctrl+o',     cmd: 'openFile',            when: 'editorFocus'},
  { key: 'ctrl+q',     cmd: 'nextTab',             when: 'editorFocus'},
  { key: 'ctrl+e',     cmd: 'encoding',            when: 'editorFocus'},
  { key: 'alt+w',      cmd: 'closeAllFile',        when: 'editorFocus'},
  { key: 'ctrl+s',     cmd: 'saveFile',            when: 'editorFocus'},

  /* key record */
  { key: 'shift+F1',   cmd: 'startEndRecordKey',   when: 'editorFocus'},
  { key: 'shift+F2',   cmd: 'execRecordKey',       when: 'editorFocus'},

  /* mouse */
  { key: 'F12',        cmd: 'mouseOpeSwitch',      when: 'editorFocus'},

  /* menu */
  { key: 'F1',         cmd: 'help',                when: 'editorFocus'},
  { key: 'alt+m',      cmd: 'openMenu',            when: 'editorFocus'},
  { key: 'alt+f',      cmd: 'openMenuFile',        when: 'editorFocus'},
  { key: 'alt+c',      cmd: 'openMenuConvert',     when: 'editorFocus'},
  { key: 'alt+e',      cmd: 'openMenuEdit',        when: 'editorFocus'},
  { key: 'alt+s',      cmd: 'openMenuSelect',      when: 'editorFocus'},

  /****************
   * promptFocus
   ****************/

  /* prompt */
  { key: 'esc',        cmd: 'escPrompt',           when: 'promptFocus'},
  { key: 'enter',      cmd: 'confirmPrompt',       when: 'promptFocus'}
  { key: 'alt+c',      cmd: 'findCaseSensitive',       when: 'promptFocus'}
  { key: 'alt+r',      cmd: 'findRegex',       when: 'promptFocus'}
]
#################################################################
```

### Key

```
// Base key
    left, right, up, down, home, end, enter, backspace, delete, pageup, pagedown, esc, F1～F12
// Modifiers key
    ctrl, shift, alt
// Modifiers key + Base key
    ctrl or shift or alt + Base key
// Modifiers key + char
    ctrl or alt + char key
```

### Command

```
// Cursor move
    CursorLeft, CursorRight, CursorUp, CursorDown, CursorRowHome, CursorRowEnd,
    CursorFileHome, CursorFileEnd, CursorPageDown, CursorPageUp,
// Select
    CursorLeftSelect, CursorRightSelect, CursorUpSelect, CursorDownSelect,
    CursorRowHomeSelect, CursorRowEndSelect, AllSelect,
// Edit
    BackTab, InsertLine, DeletePrevChar, DeleteNextChar, CutSelect, Copy, Paste, Undo, Redo,
// Find
    Find, FindNext, FindBack, Replace, MoveRow, Grep,
// File
    NewTab, NextTab, OpenFile, CloseFile, CloseAllFile, SaveFile, Encoding,
// Key record
    StartEndRecordKey, ExecRecordKey,
// Mouse
    MouseOpeSwitch
// Menu
    Help, OpenMenu, OpenMenuFile, OpenMenuConvert, OpenMenuEdit, OpenMenuSelect,
// prompt
    EscPrompt, ConfirmPrompt,
// Other
    Resize, NoBind,
```

### When

```
    editorFocus, promptFocus,
```

## Maintenance

Operation / Unexpected error log is output below.

### On Linux

/tmp/ewin/

### On Windows

%USERPROFILE%AppData\\Local\\Temp\\ewin\\

## Sample imsage

- **Initial display**  
  ![initial_display](assets/img/initial_display.png 'initial_display')
- **help**  
  ![help](assets/img/help.png 'help')
- **grep**

1. Search character, search folder, file pattern input screen.
   ![grep](assets/img/grep.png 'grep')
2. Display screen of grep result.
   ![grep_result](assets/img/grep_result.png 'grep_result')
3. Enter the Enter key on the 5th line of the grep result screen to open the target file in a new terminal.
   The cursor moves to the character in the search result.
   ![grep_result_move_file](assets/img/grep_result_move_file.png 'grep_result_move_file')

## Settings when using via Tera Term

- **Please change the keyboard setting screen opened from 「Settings」>「Keyboard」 as follows**

1. BackSpace key  
   Check 「BackSpace key」
2. Delete key  
   Uncheck「Delete key」
3. Alt key  
   Select "on"「Meta key」

- **After backing up KEYBOARD.CNF in the installation directory, make the following changes**

1. Comment out for Shift + F3・F4

```
   ;Shift + F3 key
   ;F13=573
   ;Shift + F4 key
   ;F14=574
```

2. Comment out for Ctrl + Home・End

```
   ; Ctrl + Home
   ;BuffTop=1351
   ; Ctrl + End
   ;BuffBottom=1359
```

3. Edit for Ctrl + c・v

```
   ; Ctrl + Insert
   EditCopy=1362
   ; Shift + Insert
   EditPaste=850

        ↓↓↓

   ; Ctrl + Insert
   EditCopy=off
   EditCopy=1070
   ; Shift + Insert
   EditPaste=off
   EditPaste=1071
```

4. Add for Shift + Up・Down・Right・Left・Home・End・F1..F4

```
   [User keys]
   ; PC special keys: Shift- Up, Down, Right, Left, Home, End, F1..F4
   User1=840,0,$1B[2A
   User2=848,0,$1B[2B
   User3=845,0,$1B[2C
   User4=843,0,$1B[2D
   User5=839,0,$1B[2H
   User6=847,0,$1B[2F
   User7=571,0,$1B[2P
   User8=572,0,$1B[2Q
   User9=573,0,$1B[2R
   User10=574,0,$1B[2S

   ; PC special keys: Ctrl- Home, End
   User11=1351,0,$1B[5H
   User12=1359,0,$1B[5F
```

## Future Works

- Grep-Replace function

## License

This project is distributed under [the MIT License](./LICENSE.txt).
