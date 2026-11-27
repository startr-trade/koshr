//! FEAT-002 R-2: the dual licence. FEAT-004: the community files.

use koshr_archtest::{read, repo_root};

fn non_empty(rel: &str) {
    assert!(!read(rel).trim().is_empty(), "{rel} is empty");
}

/// True when the line holds an email address, plain or obfuscated.
fn looks_like_email(line: &str) -> bool {
    let l = line.to_lowercase();
    for tok in l.split_whitespace() {
        let Some((local, domain)) = tok.split_once('@') else {
            continue;
        };
        let dotted = domain
            .find('.')
            .is_some_and(|d| d > 0 && d + 1 < domain.len());
        if !local.is_empty() && !domain.contains('@') && dotted {
            return true;
        }
    }
    // obfuscated: both the words "at" and "dot", bracketed or not
    let words: Vec<&str> = l.split(|c: char| !c.is_alphanumeric()).collect();
    words.contains(&"at") && words.contains(&"dot")
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

#[test]
fn community_files_present_linked_and_contributors_has_no_email() {
    let files = [
        "CONTRIBUTING.md",
        "SECURITY.md",
        "CODE_OF_CONDUCT.md",
        "CONTRIBUTORS",
    ];
    let readme = read("README.md");
    for f in files {
        non_empty(f);
        assert!(
            readme.contains(&format!("]({f})")),
            "README.md does not link {f}"
        );
    }
    for (n, line) in read("CONTRIBUTORS").lines().enumerate() {
        assert!(
            !looks_like_email(line),
            "CONTRIBUTORS line {} looks like an email",
            n + 1
        );
    }
}

#[test]
fn changelog_has_unreleased_heading() {
    assert!(read("CHANGELOG.md").contains("## [Unreleased]"));
}

#[test]
fn email_detector_catches_obfuscated_forms() {
    assert!(looks_like_email("jo@example.com"));
    assert!(looks_like_email("jo at example dot com"));
    assert!(looks_like_email("jo [at] example [dot] com"));
    assert!(!looks_like_email("@some-login"));
    assert!(!looks_like_email("# No email addresses are recorded here."));
}
