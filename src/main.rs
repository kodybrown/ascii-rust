use std::io::{self, IsTerminal, Write};
use std::num::NonZeroUsize;
use std::process::ExitCode;

use clap::Parser;
use crossterm::{event, terminal};

/// Display a CP437 character chart and box-drawing examples.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
  /// Chart columns (default: fit the terminal, or 9 when redirected)
  #[arg(short, long)]
  columns: Option<NonZeroUsize>,

  /// Hide the character chart
  #[arg(long)]
  no_chart: bool,

  /// Show box-drawing examples after the chart
  #[arg(short, long)]
  tables: bool,

  /// Wait for a key before exiting (interactive terminals only)
  #[arg(short, long)]
  pause: bool,
}

fn columns_for_width(width: u16) -> NonZeroUsize {
  NonZeroUsize::new(usize::from(width.saturating_sub(1) / 8)).unwrap_or(NonZeroUsize::MIN)
}

fn chart_characters() -> impl Iterator<Item = char> {
  (0_u8..=127)
    .map(|code| match code {
      0..=31 | 127 => ' ',
      _ => char::from(code),
    })
    .chain(include_str!("cp437.txt").chars())
}

fn write_chart(out: &mut impl Write, columns: NonZeroUsize) -> io::Result<()> {
  let columns = columns.get();
  for (code, ch) in chart_characters().enumerate() {
    write!(out, "{code:03} {ch}   ")?;
    if (code + 1).is_multiple_of(columns) {
      writeln!(out)?;
    }
  }
  if !256_usize.is_multiple_of(columns) {
    writeln!(out)?;
  }
  Ok(())
}

struct RawMode;

impl RawMode {
  fn enter() -> io::Result<Self> {
    terminal::enable_raw_mode()?;
    Ok(Self)
  }
}

impl Drop for RawMode {
  fn drop(&mut self) {
    let _ = terminal::disable_raw_mode();
  }
}

fn pause(out: &mut impl Write) -> io::Result<()> {
  let _raw_mode = RawMode::enter()?;
  write!(out, "Press any key to exit: ")?;
  out.flush()?;
  while !event::read()?.is_key_press() {}
  write!(out, "\r                      \r")?;
  out.flush()
}

fn run(args: Args) -> io::Result<()> {
  let stdout = io::stdout();
  let interactive_output = stdout.is_terminal();
  let mut out = stdout.lock();
  if !args.no_chart {
    let columns = args.columns.unwrap_or_else(|| {
      let width =
        if interactive_output { terminal::size().ok().map(|(width, _)| width) } else { None };
      columns_for_width(width.filter(|width| *width > 0).unwrap_or(80))
    });
    write_chart(&mut out, columns)?;
  }
  if args.tables {
    write!(out, "{}", include_str!("tables.txt"))?;
  }
  out.flush()?;
  if args.pause && interactive_output && io::stdin().is_terminal() {
    pause(&mut out)?;
  }
  Ok(())
}

fn main() -> ExitCode {
  match run(Args::parse()) {
    Ok(()) => ExitCode::SUCCESS,
    Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
    Err(error) => {
      eprintln!("ascii: {error}");
      ExitCode::FAILURE
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn automatic_columns_handle_narrow_terminals() {
    for (width, expected) in [(0, 1), (1, 1), (8, 1), (80, 9), (160, 19)] {
      assert_eq!(columns_for_width(width).get(), expected);
    }
  }

  #[test]
  fn cp437_has_all_characters_without_terminal_controls() {
    let chars: Vec<char> = chart_characters().collect();
    assert_eq!(chars.len(), 256);
    assert!(chars.iter().all(|ch| !ch.is_control()));
    assert_eq!(chars[65], 'A');
    assert_eq!(chars[128], 'Ç');
    assert_eq!(chars[179], '│');
    assert_eq!(chars[219], '█');
    assert_eq!(chars[255], '\u{a0}');
  }
}
