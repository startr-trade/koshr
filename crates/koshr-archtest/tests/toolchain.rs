//! The pinned toolchain (FEAT-002 R-1, R-3).

use koshr_archtest::{read, repo_root};

fn is_exact_version(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

fn inherits(t: &toml::Table, key: &str) -> bool {
    t.get("package")
        .and_then(|p| p.get(key))
        .and_then(|v| v.get("workspace"))
        .and_then(|w| w.as_bool())
        == Some(true)
}

/// R-1: the toolchain is pinned exactly, rustc matches it, the workspace rust-version matches
/// its major.minor, and every crate inherits version, edition and rust-version.
#[test]
fn workspace_builds_and_tests_on_pinned_toolchain() {
    let toml: toml::Table = read("rust-toolchain.toml").parse().expect("valid TOML");
    let channel = toml["toolchain"]["channel"]
        .as_str()
        .expect("toolchain.channel is a string");
    assert!(
        is_exact_version(channel),
        "channel must be an exact x.y.z, got {channel}"
    );

    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let out = std::process::Command::new(rustc)
        .arg("--version")
        .output()
        .expect("run rustc --version");
    let version = String::from_utf8(out.stdout).expect("utf-8 version");
    assert!(
        version.starts_with(&format!("rustc {channel}")),
        "rustc is `{}`, expected rustc {channel}",
        version.trim()
    );

    let root: toml::Table = read("Cargo.toml").parse().expect("valid root manifest");
    let rust_version = root["workspace"]["package"]["rust-version"]
        .as_str()
        .expect("workspace.package.rust-version is a string");
    let major_minor = channel.rsplit_once('.').expect("x.y.z").0;
    assert_eq!(
        rust_version, major_minor,
        "workspace rust-version must match the channel's major.minor"
    );

    let crates = repo_root().join("crates");
    for entry in std::fs::read_dir(&crates).expect("read crates/") {
        let dir = entry.expect("dir entry").path();
        if !dir.is_dir() {
            continue;
        }
        let manifest = std::fs::read_to_string(dir.join("Cargo.toml"))
            .unwrap_or_else(|e| panic!("{} has no Cargo.toml: {e}", dir.display()));
        let t: toml::Table = manifest.parse().expect("valid crate manifest");
        for key in ["version", "edition", "rust-version"] {
            assert!(
                inherits(&t, key),
                "{} must use {key}.workspace = true",
                dir.display()
            );
        }
    }
}

/// R-3: line endings are forced to LF.
#[test]
fn gitattributes_forces_lf() {
    assert!(
        read(".gitattributes")
            .lines()
            .any(|l| l == "* text=auto eol=lf"),
        ".gitattributes must contain `* text=auto eol=lf`"
    );
}
