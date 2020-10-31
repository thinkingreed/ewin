ewin
====

[ewin]h Simple editor for Winodws users.
  There is no need to learn new operation commands.

It provides basic features as a minimal text editor:

- Open/Save text files
- Create new text files and empty text buffer on memory
- Edit a text (put/delete characters, insert/delete lines, ...)
- Support editing UTF-8 characters
- Resizing terminal window supported. Screen size is responsible

## Installation

### On Ubuntu

_... and other Debian-based Linux distributions_

Download the latest .deb package from the [release page](https://github.com/thinkingreed/ewin/releases) and install it via:

```
sudo apt install ewin_0.5.1_amd64.deb 
```

### Snap

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


## Edit Text

ewin is a mode-less text editor. Like other famous mode-less text editors such as Nano, Emacs, you can edit text in terminal window using a keyboard.
And several keys with Ctrl or Alt modifiers are mapped to various features.

- **Operations**

| Mapping                             | Description                        |
|-------------------------------------|------------------------------------|
| `Ctrl` + `w`                        | Quit.                              |
| `Ctrl` + `s`                        | Save current buffer to file.       |
| `Ctrl` + `f`                        | Enter the character・string to search.     |


- **Moving cursor**

| Mapping                             | Description                        |
|-------------------------------------|------------------------------------|
| `↑` or `Mouse ScrollUp`             | Move cursor up.                    |
| `↓` or `Mouse ScrollDown`           | Move cursor down.                  |
| `→`                                 | Move cursor right.                 |
| `←`                                 | Move cursor left.                  |
| `HOME`                              | Move cursor to head of line.       |
| `END`                               | Move cursor to end of line.        |
| `PAGE DOWN`                         | Next page.                         |
| `PAGE UP`                           | Previous page.                     |
| `Ctrl` + `HOME`                     | Move cursor to first of line.      |
| `Ctrl` + `END`                      | Move cursor to last of line.       |

- **Edit text**

| Mapping                 | Description               |
|-------------------------|---------------------------|
| `Enter`                 | Insert new line           |
| `BACKSPACE`             | Delete character          |
| `DELETE`                | Delete next character     |
| `Ctrl` + `x`            | Select range cut.         |
| `Ctrl` + `c`            | Select range cop.         |
| `Ctrl` + `v`            | Paste the copied characters.|
| `Ctrl` + `r`            | Replace character.|
| `Ctrl` + `z`            | Undo.Undo the last edit and return to the original state.|
| `Ctrl` + `y`            | Redo.Make the last update again.|


- **Select text**

| Mapping                 | Description               |
|-------------------------|----------------------------------------------------------------------------------|
| `Shift` + `↑`           | Select from the beginning of the current line and the end of the line above.     |
| `Shift` + `↓`           | Select from the end of the current line and the beginning of the line below.     |
| `Shift` + `→`           | Select the next character.　　　　　　　　　　　　　　　　　　　　　　　　　　 　　 　| 
| `Shift` + `←`           | Select the previous character.                                                   | 
| `Shift` + `HOME`        | Select the head of line.                                                         | 
| `Shift` + `END`         | Select the end of line.                                                          |
| `Ctrl` + `a`            | Select all.                                                                      | 
| `F3`                    | Search for characters below.     　　　　　                                       |
| `Shift` + `F4`          | Search for above characters below.　　　　　                                      |
| `Mouse` + `Left.Down, Drag, Up`    | Select a range.|

## Future Works

- Grep function
- WASI support

## License

This project is distributed under [the MIT License](./LICENSE.txt).
