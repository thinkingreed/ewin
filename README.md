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

| Mapping        | Motion                                                                               |Key recording|
|----------------|--------------------------------------------------------------------------------------|-------------|
| `Ctrl` + `w`   | Quit.                                                                                |―            |
| `Ctrl` + `s`   | Save current buffer to file.                                                         |―            |
| `F1`           | Key binding display at the bottom of the screen.                                     |―            |
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
| `Ctrl` + `l`                        | Move cursor to specified line.     |―            |

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

## Setting file

Please edit the contents of setting.toml(initial setting) below and put it in this location.

### On Linux
$HOME/.config/ewin/setting.toml



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


## Sample imsage
  
- **Initial display**  
![initial_display](assets/img/initial_display.png "initial_display")
  
- **help**  
![help](assets/img/help.png "help")
  
- **grep**  
1. Search character, search folder, file pattern input screen.
![grep](assets/img/grep.png "grep")  
2. Display screen of grep result.
![grep_result](assets/img/grep_result.png "grep_result")
3. Enter the Enter key on the 5th line of the grep result screen to open the target file in a new terminal.
   The cursor moves to the character in the search result.
![grep_result_move_file](assets/img/grep_result_move_file.png "grep_result_move_file")

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

- Support for character codes other than UTF-8
- Grep-Replace function

## License

This project is distributed under [the MIT License](./LICENSE.txt).
