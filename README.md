![The Cryptum Text logo, it shows a stylised text input caret, and to it's right are some smaller characters consisting of a forward slash, two asterisks, and another forward slash](./data/icons/cryptum-text-icon.svg)

# Cryptum Text

Cryptum Text is a GTK based text/code editor written in Rust and powered by GTK-RS, Relm4, Sourceview5, and Libadwaita.

![A screenshot of Cryptum Text with a file list open, and the main editor content being it's own source code.](./data/cryptum-text-image.png)
(Above: Cryptum Text with it's own source code, as is tradition.)

# Features
- Syntax Highlighting
- A Mini-Map
- A Status Bar
- File List
- Saving Settings Between Restarts

## In Progress
- File List Context Menu
- Preferences Dialog

## Potential Future Features (Not Confirmed to be Implemented)
- Autocompletion
- File Tree
- Git Intergration
- Mini-Map Pane Highlight
- Confirmation Dialog(s)

# Extra Notes
- The editor widget specifically cannot auto adjust to the system theme, however you can toggle the theme manually and it will persist across restarts (provided "editor_theme" option in the settings file hasn't been changed)
- While the program might run on Windows or MacOS, I haven't tested this myself.
- The ability to toggle showing hidden files is only available on UNIX systems, due to complications with detecting hidden files on Windows
- Folders will have have a / character at the end of them in the file list to differentiate them from files

# License
This project it licensed under the GNU General Public License Version 3. See the [LICENSE](./LICENSE) file for details (specifically the file in the root directory with the README, NOT any of the files in the `licenses/` directory).
