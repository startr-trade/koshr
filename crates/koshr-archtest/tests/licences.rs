//! FEAT-002 R-2: the dual licence.

use koshr_archtest::{read, repo_root};

fn non_empty(rel: &str) {
    assert!(!read(rel).trim().is_empty(), "{rel} is empty");
}

#[test]
fn every_crate_declares_dual_licence() {
    non_empty("LICENSE-MIT");
    non_empty("LICENSE-APACHE");
    let root: toml::Table = toml::from_str(&read("Cargo.toml")).expect("root manifest parses");
    let ws = root["workspace"]["package"]["license"]
        .as_str()
        .expect("workspace.package.license is set");
    assert_eq!(ws, "MIT OR Apache-2.0");
    let mut seen = 0;
    for entry in std::fs::read_dir(repo_root().join("crates")).expect("crates/ exists") {
        let manifest = entry.expect("dir entry").path().join("Cargo.toml");
        if !manifest.exists() {
            continue;
        }
        seen += 1;
        let m: toml::Table =
            toml::from_str(&std::fs::read_to_string(&manifest).expect("manifest reads"))
                .expect("manifest parses");
        let lic = &m["package"]["license"];
        let inherits = lic.get("workspace").and_then(toml::Value::as_bool) == Some(true);
        assert!(
            inherits,
            "{} lacks license.workspace = true",
            manifest.display()
        );
    }
    assert!(seen > 0, "no crate manifests found");
}
