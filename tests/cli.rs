use std::process::{Command, Output};

fn ascii(args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_ascii")).args(args).output().expect("ascii should run")
}

fn output(args: &[&str]) -> String {
  let result = ascii(args);
  assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
  assert!(result.stderr.is_empty());
  String::from_utf8(result.stdout).expect("output should be UTF-8")
}

#[test]
fn redirected_chart_is_complete_and_safe() {
  let text = output(&[]);
  assert_eq!(text.lines().count(), 29);
  assert!(text.chars().all(|ch| ch == '\n' || !ch.is_control()));
  let one_column = output(&["--columns", "1"]);
  let rows: Vec<&str> = one_column.lines().collect();
  assert_eq!(rows.len(), 256);
  for (code, row) in rows.iter().enumerate() {
    assert!(row.starts_with(&format!("{code:03} ")));
  }
  assert_eq!(rows[65], "065 A   ");
  assert_eq!(rows[179], "179 │   ");
  assert_eq!(rows[255], "255 \u{a0}   ");
}

#[test]
fn explicit_columns_and_large_values_work() {
  assert_eq!(output(&["-c", "8"]).lines().count(), 32);
  assert_eq!(output(&["--columns=300"]).lines().count(), 1);
}

#[test]
fn tables_are_preserved_and_flags_can_be_combined_in_any_order() {
  let expected = include_str!("../src/tables.txt");
  assert_eq!(output(&["--no-chart", "--tables"]), expected);
  assert_eq!(output(&["--tables", "-c", "8"]), output(&["-c", "8", "--tables"]));
  assert!(output(&["--no-chart"]).is_empty());
}

#[test]
fn help_and_version_do_not_print_a_chart() {
  let help = output(&["--help"]);
  assert!(help.contains("--columns"));
  assert!(!help.contains("065 A"));
  assert_eq!(output(&["--version"]), format!("ascii {}\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn invalid_arguments_fail_without_chart_output() {
  for args in
    [vec!["--columns"], vec!["-c", "0"], vec!["-c", "-1"], vec!["-c", "abc"], vec!["--unknown"]]
  {
    let result = ascii(&args);
    assert_eq!(result.status.code(), Some(2));
    assert!(result.stdout.is_empty());
    assert!(!result.stderr.is_empty());
  }
}

#[test]
fn pause_does_not_block_redirected_output() {
  assert_eq!(output(&["--pause"]), output(&[]));
}
