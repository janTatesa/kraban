# kraban

This project is heavily inspired by <https://github.com/GabAlpha/basilk/>
## Features
- [ ] Git sync
- [ ] Task due dates
- [ ] Stats
## Configuration
Upon first start, kraban will automatically create the config file in `CONFIG_DIRECTORY/kraban/kraban.toml`. All non-obvious options are explained in the comments. Colors can either be specified with number name or hex
## Importing from basilk
To import from basilk, copy the basilks state file into `STATE_DIRECTORY/kraban/tasks.json`. In linux, the state directory is `.local/state`
## Rename columns
There is currently no way to rename columns, do search and replace in the state file
## Keybindings
Most keybindings are shown in the app. These keybindings are used when entering text
| Mappings                                     | Description                               |
|----------------------------------------------|-------------------------------------------|
| `Ctrl+H`, `Backspace`                        | Delete one character before cursor        |
| `Ctrl+D`, `Delete`                           | Delete one character next to cursor       |
| `Ctrl+K`                                     | Delete from cursor until the end of line  |
| `Ctrl+J`                                     | Delete from cursor until the head of line |
| `Ctrl+W`, `Alt+H`, `Alt+Backspace`           | Delete one word before cursor             |
| `Alt+D`, `Alt+Delete`                        | Delete one word next to cursor            |
| `Ctrl+U`                                     | Undo                                      |
| `Ctrl+R`                                     | Redo                                      |
| `Ctrl+Y`                                     | Paste yanked text                         |
| `Ctrl+F`, `→`                                | Move cursor forward by one character      |
| `Ctrl+B`, `←`                                | Move cursor backward by one character     |
| `Ctrl+P`, `↑`                                | Move cursor up by one line                |
| `Ctrl+N`, `↓`                                | Move cursor down by one line              |
| `Alt+F`, `Ctrl+→`                            | Move cursor forward by word               |
| `Atl+B`, `Ctrl+←`                            | Move cursor backward by word              |
| `Ctrl+E`, `End`, `Ctrl+Alt+F`, `Ctrl+Alt+→`  | Move cursor to the end of line            |
| `Ctrl+A`, `Home`, `Ctrl+Alt+B`, `Ctrl+Alt+←` | Move cursor to the head of line           |
## License

Copyright (c) Tatesa Uradnik <taduradnik@proton.me>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
