# ascii

A Rust command-line tool that prints a numbered CP437 character chart and
box-drawing examples.

## Build and run

Install a current stable Rust toolchain, then run these commands from the
project directory:

```sh
cargo build --release --locked
cargo run --release --locked
cargo run --release --locked -- --columns 8
cargo run --release --locked -- --no-chart --tables
cargo run --release --locked -- --help
```

Using `cargo run` finds the executable even when you configure a custom build
output directory. Arguments after `--` are passed to `ascii`.

To install the executable into Cargo's bin directory (normally `~/.cargo/bin`):

```sh
cargo install --path . --locked
ascii --no-chart --tables
```

Ensure that Cargo's bin directory is on your `PATH`. Only the executable is
needed at runtime; no .NET runtime or separate data files are required.

## Options

| Option            | Behavior                                                                              |
| ----------------- | ------------------------------------------------------------------------------------- |
| `-c, --columns N` | Set a positive number of chart columns. Omit for automatic sizing.                    |
| `--no-chart`      | Hide the character chart. (Useful to only show --tables.)                                                            |
| `-t, --tables`    | Show box-drawing examples after the chart. Combine with `--no-chart` for tables only. |
| `-p, --pause`     | Wait for one key before exiting, only when both input and output are terminals.       |
| `-h, --help`      | Print help and exit.                                                                  |
| `-V, --version`   | Print the version and exit.                                                           |

With no options, the app prints the chart. `--no-chart` alone produces no output.
Zero is not a valid column count. Windows-style `/flag` and negated `-!flag`
syntax are not supported.

## Output

The chart contains decimal codes 0–255. Printable ASCII and extended CP437
characters are emitted as UTF-8; control characters (0–31 and 127) appear as
blanks so they cannot alter the terminal. Use a terminal font with box-drawing
support.

Chart columns fit the terminal by default, with a minimum of one column.
Redirected output, or an unavailable terminal width, uses nine columns.
Piping into tools such as `head` exits cleanly when the pipe closes.

The table examples include single-line, double-line, mixed-line, and block
styles. Each main set shows four shapes: outline only, a horizontal divider,
a horizontal divider with the bottom half split, and a four-cell grid. Shadow
examples follow. Table layouts are fixed; `--columns` affects only the chart.

## Build output directory

To keep generated files outside the source tree, set `CARGO_TARGET_DIR` or
create a local `.cargo/config.toml`, for example:

```toml
[build]
target-dir = "/tmp/_rust/Projects/ascii/ascii-rust/target"
```

Choose the path for your checkout. `.cargo/config.toml` is ignored by Git so
this machine-specific setting stays local. Without an override, Cargo uses
`target/` in the project directory. Files under `/tmp` may be cleared by the
system; Cargo rebuilds them as needed.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

Commit `Cargo.toml` and `Cargo.lock`; generated build output is ignored.

The data files in `src/` are embedded into the executable at compile time:

- `tables.txt` contains the box-drawing examples printed by `--tables`.
- `cp437.txt` contains the 128 Unicode characters for CP437 codes 128–255,
  in code order. Do not add a trailing newline; every character is an entry.

Keep both files in the repository and rebuild after editing them.

Argument parsing uses `clap`; terminal sizing and key input use `crossterm`.
Linux has been verified locally; Windows and macOS have not been tested.

## Author and license

Kody Brown <thewizard@wasatchwizard.com>.

Source: [kodybrown/ascii-rust](https://github.com/kodybrown/ascii-rust).
Licensed under the [MIT License](LICENSE).
