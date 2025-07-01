# repl-logic-guest

This crate contains the logic of the REPL:

- Parses user input into commands and payloads
- Expands environment variables in command arguments (e.g., `$HOME` → `/home/user`)
- Manages reserved commands that cannot be overridden by plugins:
  - `export <key>=<value>` - Sets environment variables
  - `help <command>` - Shows command documentation
  - `list-commands` - Lists available plugins and reserved commands
- Provides manual pages for reserved commands via the `man` command
- Routes non-reserved commands to plugins for execution from the host (cli or web)

## Notes

This crate was initialized with `cargo component new`.

The building process is handled by the [`justfile`](../../justfile) in the root of the project.
