# ascii

A Rust command-line tool that prints a numbered CP437 character chart and
box-drawing examples.

## Sample Output

```sh
 λ ascii --tables
000     001     002     003     004     005     006
007     008     009     010     011     012     013
014     015     016     017     018     019     020
021     022     023     024     025     026     027
028     029     030     031     032     033 !   034 \"
035 #   036 $   037 %   038 &   039 '   040 (   041 )
042 *   043 +   044 ,   045 -   046 .   047 /   048 0
049 1   050 2   051 3   052 4   053 5   054 6   055 7
056 8   057 9   058 :   059 ;   060 <   061 =   062 >
063 ?   064 @   065 A   066 B   067 C   068 D   069 E
070 F   071 G   072 H   073 I   074 J   075 K   076 L
077 M   078 N   079 O   080 P   081 Q   082 R   083 S
084 T   085 U   086 V   087 W   088 X   089 Y   090 Z
091 [   092 \   093 ]   094 ^   095 _   096 \`   097 a
098 b   099 c   100 d   101 e   102 f   103 g   104 h
105 i   106 j   107 k   108 l   109 m   110 n   111 o
112 p   113 q   114 r   115 s   116 t   117 u   118 v
119 w   120 x   121 y   122 z   123 {   124 |   125 }
126 ~   127     128 Ç   129 ü   130 é   131 â   132 ä
133 à   134 å   135 ç   136 ê   137 ë   138 è   139 ï
140 î   141 ì   142 Ä   143 Å   144 É   145 æ   146 Æ
147 ô   148 ö   149 ò   150 û   151 ù   152 ÿ   153 Ö
154 Ü   155 ¢   156 £   157 ¥   158 ₧   159 ƒ   160 á
161 í   162 ó   163 ú   164 ñ   165 Ñ   166 ª   167 º
168 ¿   169 ⌐   170 ¬   171 ½   172 ¼   173 ¡   174 «
175 »   176 ░   177 ▒   178 ▓   179 │   180 ┤   181 ╡
182 ╢   183 ╖   184 ╕   185 ╣   186 ║   187 ╗   188 ╝
189 ╜   190 ╛   191 ┐   192 └   193 ┴   194 ┬   195 ├
196 ─   197 ┼   198 ╞   199 ╟   200 ╚   201 ╔   202 ╩
203 ╦   204 ╠   205 ═   206 ╬   207 ╧   208 ╨   209 ╤
210 ╥   211 ╙   212 ╘   213 ╒   214 ╓   215 ╫   216 ╪
217 ┘   218 ┌   219 █   220 ▄   221 ▌   222 ▐   223 ▀
224 α   225 ß   226 Γ   227 π   228 Σ   229 σ   230 µ
231 τ   232 Φ   233 Θ   234 Ω   235 δ   236 ∞   237 φ
238 ε   239 ∩   240 ≡   241 ±   242 ≥   243 ≤   244 ⌠
245 ⌡   246 ÷   247 ≈   248 °   249 ∙   250 ·   251 √
252 ⁿ   253 ²   254 ■   255  

 λ ascii --tables --no-chart
┌───────┐   ┌───────┐   ┌───────┐   ┌───┬───┐
│       │   │       │   │       │   │   │   │
│       │   ├───────┤   ├───┬───┤   ├───┼───┤
│       │   │       │   │   │   │   │   │   │
└───────┘   └───────┘   └───┴───┘   └───┴───┘
╔═══════╗   ╔═══════╗   ╔═══════╗   ╔═══╦═══╗
║       ║   ║       ║   ║       ║   ║   ║   ║
║       ║   ╠═══════╣   ╠═══╦═══╣   ╠═══╬═══╣
║       ║   ║       ║   ║   ║   ║   ║   ║   ║
╚═══════╝   ╚═══════╝   ╚═══╩═══╝   ╚═══╩═══╝
╓───────╖   ╓───────╖   ╓───────╖   ╓───┬───╖
║       ║   ║       ║   ║       ║   ║   │   ║
║       ║   ╟───────╢   ╟───┬───╢   ╟───┼───╢
║       ║   ║       ║   ║   │   ║   ║   │   ║
╙───────╜   ╙───────╜   ╙───┴───╜   ╙───┴───╜
╓───────╖   ╓───────╖   ╓───────╖   ╓───╥───╖
║       ║   ║       ║   ║       ║   ║   ║   ║
║       ║   ╟───────╢   ╟───╥───╢   ╟───╫───╢
║       ║   ║       ║   ║   ║   ║   ║   ║   ║
╙───────╜   ╙───────╜   ╙───╨───╜   ╙───╨───╜
╒═══════╕   ╒═══════╕   ╒═══════╕   ╒═══╤═══╕
│       │   │       │   │       │   │   │   │
│       │   ╞═══════╡   ╞═══╤═══╡   ╞═══╪═══╡
│       │   │       │   │   │   │   │   │   │
╘═══════╛   ╘═══════╛   ╘═══╧═══╛   ╘═══╧═══╛

█████████   █████████   █████████   █████████
█       █   █       █   █       █   █   █   █
█       █   █████████   █████████   █████████
█       █   █       █   █   █   █   █   █   █
█████████   █████████   █████████   █████████

█▀▀▀▀▀▀▀█   █▀▀▀▀▀▀▀█   █▀▀▀▀▀▀▀█   █▀▀▀█▀▀▀█
█       █   █       █   █       █   █   █   █
█       █   █████████   █████████   █████████
█       █   █       █   █   █   █   █   █   █
█▄▄▄▄▄▄▄█   █▄▄▄▄▄▄▄█   █▄▄▄█▄▄▄█   █▄▄▄█▄▄▄█

█▀▀▀▀▀▀▀█   █▀▀▀▀▀▀▀█   █▀▀▀▀▀▀▀█   █▀▀▀█▀▀▀█
█       █   █       █   █       █   █   █   █
█       █   █▄▄▄▄▄▄▄█   █▄▄▄▄▄▄▄█   █▄▄▄█▄▄▄█
█       █   █       █   █   █   █   █   █   █
█▄▄▄▄▄▄▄█   █▄▄▄▄▄▄▄█   █▄▄▄█▄▄▄█   █▄▄▄█▄▄▄█

█▀▀▀▀▀▀▀█   █▀▀▀▀▀▀▀█   █▀▀▀▀▀▀▀█   █▀▀▀█▀▀▀█
█       █   █       █   █       █   █   █   █
█       █   █▀▀▀▀▀▀▀█   █▀▀▀█▀▀▀█   █▀▀▀█▀▀▀█
█       █   █       █   █   █   █   █   █   █
█▄▄▄▄▄▄▄█   █▄▄▄▄▄▄▄█   █▄▄▄█▄▄▄█   █▄▄▄█▄▄▄█

┌───┬───┐   ┌───┬───┐
│   │   │▄  │   │   │▓
├───┼───┤█  ├───┼───┤▓
│   │   │█  │   │   │▓
└───┴───┘█  └───┴───┘▓
  ▀▀▀▀▀▀▀▀   ▓▓▓▓▓▓▓▓▓
```

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
