ewin
====

***Simple editor for Window(GUI) users.***

***No need to remember commands***

It provides basic features as a minimal text editor:

- Open/Save text files
- Create new text files and empty text buffer on memory
- Edit a text (put/delete characters, insert/delete lines, ...)
- Support editing UTF-8 characters
- Resizing terminal window supported. Screen size is responsible
- Mouse support

## Installation

### On Ubuntu

_... and other Debian-based Linux distributions_

Download the latest .deb package from the [release page](https://github.com/thinkingreed/ewin/releases) and install it via:

### On CentOS

Download the latest .rpm package from the [release page](https://github.com/thinkingreed/ewin/releases) and install it via:

```
sudo yun install ewin_0.0.0.x86_64.rpm
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

| Mapping        | Motion                                                                               |Key recording|
|----------------|--------------------------------------------------------------------------------------|-------------|
| `Ctrl` + `w`   | Quit.                                                                                |―            |
| `Ctrl` + `s`   | Save current buffer to file.                                                         |―            |
| `Ctrl` + `f`   | Enter the characters to incremental search.Search target is open file.               |―            |
| `Ctrl` + `g`   | Grep.Enter the characters to search.The search target is the entered file pattern    |―            |
|                | Command to use : grep -rHnI search_str --include=search_filenm search_folder         |             |
|                | -r:Subfolder search,-H:File name display,-n:Line number display,-I: Binary file not applicable|             |
| `Shift` + `F1` | Key record start or stop.Recording key operation.                                    |―            |
| `Shift` + `F2` | Execution of the recorded key.                                                       |―            |


- **Moving cursor**

| Mapping                             | Motion                             |Key recording|
|-------------------------------------|------------------------------------|-------------|
| `↑` or `Mouse ScrollUp`             | Move cursor up.                    |target       |
| `↓` or `Mouse ScrollDown`           | Move cursor down.                  |target       |
| `→`                                 | Move cursor right.                 |target       |
| `←`                                 | Move cursor left.                  |target       |
| `HOME`                              | Move cursor to head of line.       |target       |
| `END`                               | Move cursor to end of line.        |target       |
| `PAGE DOWN`                         | Next page.                         |target       |
| `PAGE UP`                           | Previous page.                     |target       |
| `Ctrl` + `HOME`                     | Move cursor to first of line.      |target       |
| `Ctrl` + `END`                      | Move cursor to last of line.       |target       |

- **Edit text**

| Mapping                 | Motion                          |Key recording|
|-------------------------|---------------------------------|-------------|
| `Enter`                 | Insert new line                 |target       |
| `BACKSPACE`             | Delete character                |target       |
| `DELETE`                | Delete next character           |target       |
| `Ctrl` + `x`            | Select range cut.               |target       |
| `Ctrl` + `c`            | Select range cop.               |target       |
| `Ctrl` + `v`            | Paste the copied characters.    |target       |
| `Ctrl` + `r`            | Replace character.              |―            |
| `Ctrl` + `z`            | Undo.Undo the last edit.        |―            |
| `Ctrl` + `y`            | Redo.Make the last update again.|―            |


- **Select text**

| Mapping                   | Motion                                                                     |Key recording|
|---------------------------|----------------------------------------------------------------------------|-------------|
| `Shift` + `↑`             | Select from the beginning of the current line and the end of the line above|target       |
| `Shift` + `↓`             | Select from the end of the current line and the beginning of the line below|target       |
| `Shift` + `→`             | Select the next character.　　　　　　　　　　　　　　　　　　　　　　　　　　  |target       |
| `Shift` + `←`             | Select the previous character.                                             |target       | 
| `Shift` + `HOME`          | Select the head of line.                                                   |target       | 
| `Shift` + `END`           | Select the end of line.                                                    |target       |
| `Ctrl` + `a`              | Select all.                                                                |target       |
| `F3`                      | Search for characters below.     　　　　　                                 |target       |
| `Shift` + `F4`            | Search for above characters below.　　　　　                                |target       |
| `Mouse Left.Down` + `Drag`| Select a range.                                                            |―            |
| `Mouse Double click`      | Select a range.Delimiter is ```! 　"\#$%&()*+-',./:;<=>?@[]^`{|}~```       |―            |
| `Mouse Triple click`      | Select a line.                                                             |―            |


## Operation restrictions
| motion        | Mapping          | environment     |Contents                                           |
|---------------|------------------|-----------------|---------------------------------------------------|
|`Grep`         | `Ctrl` + `g`     | WSL             | Set the distribution with ewin installed to WSL default|
|               |                  | Ubuntu・CentOS  | Only when using "gnome-terminal"                  |
|`Key record`   | `Ctrl` + `F1`    | WSL             | keybindings.command."copy" and "paste" needs to be changed to something other than Ctrl+c, Ctrl+v. Ex)Ctrl+Shift+c, Ctrl+Shift+v                                 |
|`Save`    　   | `Ctrl` + `s`     | All             | Forcibly convert CR + LF of line feed code to LF  |
|`Copy`・`Paste`| `Ctrl` + `c`・`v`| WSL             | Need path to powershell.exe. Try $PSHOME at PowerShell terminal|



## Future Works

- Making various settings into a configuration file
- Grep-Replace function

## License

This project is distributed under [the MIT License](./LICENSE.txt).
