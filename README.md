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
- Mocro support at Javascript
- Context menu support
- Tab support
- Box select・Inseret support

[![Rust](https://github.com/thinkingreed/ewin/actions/workflows/ci.yaml/badge.svg)](https://github.com/thinkingreed/ewin/actions/workflows/ci.yaml)

## Installation

### On Ubuntu

_... and other Debian-based Linux distributions_

Download the latest .deb package from the [release page](https://github.com/thinkingreed/ewin/releases) and install it via:

### On CentOS

Download the latest .rpm package from the [release page](https://github.com/thinkingreed/ewin/releases) and install it via:

```
rpm -ivh ewin_0.0.0.x86_64.rpm
```

### On Windows

Download the latest .exe package from the [release page](https://github.com/thinkingreed/ewin/releases) and install it via:

## Usage

### CLI

Please see `ewin --help` for command usage.

#### File edit

Installing package introduces `ew` command in your system.

```sh
$ ew [file]         # Open files to edit
```

#### Output config file

Configuration file output.

```sh
$ ewin -o, --output-config
```

The output location of the config file is as follows.

#### On Linux

\$HOME/.config/ewin/

#### On Windows

%USERPROFILE%\AppData\Roaming\ewin\

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
| `Alt` + `←`・`→`   | Switch tab.                                                                                                    | ―             |
| `Ctrl` + `e`   | Specify the character code and reload. Or set the character code line feed code and BOM.                            | ―             |
| `F12`          | Mouse capture changes.Used for clipboard access via terminal app                                                    | ―             |
|                | when connecting to a remote terminal                                                                                | ―             |
| `F10`          | Display Context menu.                                                                                               | ―             |

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

| Mapping                    | Motion                                                                                        | Key recording |
| -------------------------- | --------------------------------------------------------------------------------------------- | ------------- |
| `Shift` + `↑`              | Select from the beginning of the current line and the end of the line above                   | target        |
| `Shift` + `↓`              | Select from the end of the current line and the beginning of the line below                   | target        |
| `Shift` + `→`              | Select the next character.　　　　　　　　　　　　　　　　　　　　　　　　　　                | target        |
| `Shift` + `←`              | Select the previous character.                                                                | target        |
| `Shift` + `HOME`           | Select the head of line.                                                                      | target        |
| `Shift` + `END`            | Select the end of line.                                                                       | target        |
| `Ctrl` + `a`               | Select all.                                                                                   | target        |
| `F3`                       | Search for characters below. 　　　　　                                                       | target        |
| `Shift` + `F4`             | Search for above characters below.　　　　　                                                  | target        |
| `Mouse Left.Down` + `Drag` | Select a range.                                                                               | ―             |
| `Mouse Double click`       | Select a range.Delimiter is `` ! "\#$%&()*+-',./:;<=>?@[]^`{|}~ ``                            | ―             |
| `Mouse Triple click`       | Select a line.                                                                                | ―             |
| `Shift` + `F6`             | Box(rectangle) Select mode Start or Stop.                                                     | ―             |
| `Alt` + `↑`                | Box(rectangle) Select from the beginning of the current line and the end of the line above    | target        |
| `Alt` + `↓`                | Box(rectangle) Select from the end of the current line and the beginning of the line below    | target        |
| `Alt` + `→`                | Box(rectangle) Select the next character.　　　　　　　　　　　　　　　　　　　　　　　　　　 | target        |
| `Alt` + `←`                | Box(rectangle) Select the previous character.                                                 | target        |
| `Alt` + `HOME`             | Box(rectangle) Select the head of line.                                                       | target        |
| `Alt` + `END`              | Box(rectangle) Select the end of line.                                                        | target        |

## Operation restrictions

| motion                 | Mapping                | environment | Contents                                                                                                                         |
| ---------------------- | ---------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `Key record`           | `Ctrl` + `F1`          | WSL         | keybindings.command."copy" and "paste" needs to be changed to something other than Ctrl+c, Ctrl+v. Ex)Ctrl+Shift+c, Ctrl+Shift+v |
| `Copy`・`Paste`・`Cut` | `Ctrl` + `c`・`v`・`x` | WSL         | If you want to copy it to the clipboard, you need the path to powershell.exe. Try \$ PSHOME in a PowerShell terminal             |

## Setting file

When [Output config file command](#Output-config-file) is executed, the Setting configuration file is output to the following location, so it can be edited.

### On Linux

\$HOME/.config/ewin/setting.toml

### On Windows

%USERPROFILE%\AppData\Roaming\ewin\setting.toml

### Initial Setting

The initial Settings are as follows.

[Initial Setting](https://github.com/thinkingreed/ewin/wiki/setting)

## Key binding

When [Output config file command](#Output-config-file) is executed, the keybind configuration file is output to the following location, so it can be edited.

### On Linux

\$HOME/.config/ewin/keybind.json5

### On Windows

%USERPROFILE%\AppData\Roaming\ewin\keybind.json5

### Initial key bind

The initial key bind settings are as follows.

[Initial key bind](https://github.com/thinkingreed/ewin/wiki/Key-binding)

## Maintenance

Operation / Unexpected error log is output below.

### On Linux

/tmp/ewin/

### On Windows

%USERPROFILE%AppData\\Local\\Temp\\ewin\\

## Sample imsage

- **Initial display**  
  ![initial](assets/img/init.png 'initial')
- **help**  
  ![help](assets/img/help.png 'help')
- **grep**

1. Input search character, search folder, file pattern.
2. Display screen of grep result.
3. Enter the Enter key on the 3th line of the grep result screen to open the target file in a new tab.
   The cursor moves to the character in the search result.
   ![grep](assets/img/grep.gif 'grep')

## Settings when using via Tera Term

Settings when using via Tera Term is as follows.

[Using via Tera Term](https://github.com/thinkingreed/ewin/wiki/Using-via-Tera-Term)

## Public functions for Javascript macros

Below is a list of published Javascript functions.
Built-in v8 engine using rusty_v8.

[Public functions for macros](https://github.com/thinkingreed/ewin/wiki/Public-functions-for-macros)

## Future Works

- Grep-Replace function
- Swap file

## License
This project is distributed under [the MIT License](./LICENSE).
