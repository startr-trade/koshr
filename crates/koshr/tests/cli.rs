//! Command-line behaviour of the `koshr` binary in release 0.1.0.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_koshr"))
        .args(args)
        .output()
        .expect("koshr binary runs")
}

/// R-1: `--version` prints the crate name and version, then exits 0.
#[test]
fn cli_version_prints_name_and_version() {
    let out = run(&["--version"]);
    assert_eq!(out.status.code(), Some(0));
    let expected = format!("koshr {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(String::from_utf8_lossy(&out.stdout), expected);
}

/// R-2: `--help` and bare `koshr` print the usage and exit 0.
#[test]
fn cli_help_lists_commands() {
    for args in [&["--help"][..], &[][..]] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(0), "args {args:?}");
        assert!(
            String::from_utf8_lossy(&out.stdout).contains("Usage: koshr"),
            "args {args:?}"
        );
    }
}
