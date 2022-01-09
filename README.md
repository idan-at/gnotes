# gnotes

gnotes is a simple cli that allows you to easily add and edit notes without leaving your terminal.

## Installation
1. Build from scratch:

- `git clone git@github.com:idan-at/gnotes.git`
- `cargo install`

## Commands

gnotes supports the following commands:

NOTE: For all commands, the default directory is `"notes"`.

- `gnotes new <name> [--dir <dir>] [-m message]`: Creates a new note file under the specified directory. If `-m` is passed, the message will be written to the file. Otherwise, the note will be open with your configured terminal editor.
- `gnotes add <name> <message> [--dir <dir>]`: Appends the message to the given note. If the note doesn't already exist, it will be created.
- `gnotes remove <name> [--dir <dir>]`: Removes a note file.
- `gnotes rm`: Alias for `gnotes remove`.
- `gnotes list [--dir <dir> --all --include-headers]`: Lists all the notes in the specified directory.
- `gnotes ls`: Alias for `gnotes list`.
- `gnotes show <name> [--dir <dir>]`: Shows the note content in Markdown format.
- `gnotes edit <name> [--dir <dir>]`: Opens the notes files with your terminal editor. If the note doesn't exist, it will be created.
- `gnotes save`: Save the notes to the git repository specified in the config file.
- `gnotes clone`: Clones the notes repository which is specified in the config file to the notes home directory.
- `gnotes search <tag> [--dir <dir> --all]`: Searches the notes files with the specified tag.
- `gnotes tag <name> <tags> [--dir <dir>]`: Adds tags for a specific note.
- `gnotes untag <name> <tag> [--dir <dir>]`: Removes a tag for a specific note.

## Config

`gnotes` supports an optional configuration file under `$HOME/.gnotes.toml`. Each of the configuration properties is optional.
- `notes_dir`: The directory where notes will be stored. Defaults to `$HOME/.gnotes`.
- `auto_save`: Whether to automatically save notes to git. Defaults to `false`.
- `repository`: The URL of the repository to save notes to. Defaults to `None`.

NOTE: If `auto_save` is `true`, the `repository` is mandatory.

**Each of the configuration properties can be overridden by an environment variable with the `GNOTES_` prefix. for example, if `GNOTES_NOTES_DIR` is set, it will override the `notes_dir` that is specified in the configuration file.

## Backing up your notes
`gnotes` supports backing up your notes via `git`, if the `repository` is configured.
Backup can be manually by running `gnotes save` or automatically after every notes change if `auto_save` is configured.

If you don't want to backup your notes with `git`, you can always set the home directory to your `dropbox` folder, which will automatically be backed up by `dropbox` (or any other similar system).
