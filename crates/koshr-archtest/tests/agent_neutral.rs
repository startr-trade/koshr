//! Agent-neutral product text: no vendor agent name outside the adapter files (MNT-001).

use std::fs;
use std::path::{Path, PathBuf};

use koshr_archtest::{read, repo_root};

/// Product text files at the top of the repository.
const TOP_LEVEL: &[&str] = &[
    "README.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
    "CODE_OF_CONDUCT.md",
    "CHANGELOG.md",
];

/// Recursively collect files with the given extension under `dir`.
/// A missing directory is skipped silently.
fn collect(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect(&path, ext, out);
        } else if path.extension().is_some_and(|e| e == ext) {
            out.push(path);
        }
    }
}

/// Every product text file: the top-level docs, `docs/guide/**/*.md`, `crates/*/src/**/*.rs`
/// and `.github/workflows/**/*.yml`. The adapter files (CLAUDE.md, .claude/) and the fixture
/// are never in this set.
fn product_text_files() -> Vec<PathBuf> {
    let root = repo_root();
    let mut files: Vec<PathBuf> = TOP_LEVEL
        .iter()
        .map(|f| root.join(f))
        .filter(|p| p.exists())
        .collect();
    collect(&root.join("docs/guide"), "md", &mut files);
    collect(&root.join(".github/workflows"), "yml", &mut files);
    if let Ok(crates) = fs::read_dir(root.join("crates")) {
        for krate in crates {
            collect(
                &krate.expect("crate entry").path().join("src"),
                "rs",
                &mut files,
            );
        }
    }
    files.sort();
    files
}

/// The deny list from the fixture, lowercased, one word per line.
fn vendor_words() -> Vec<String> {
    read("crates/koshr-archtest/tests/fixtures/agent-vendors.txt")
        .lines()
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Whole-word tokens: maximal runs of letters, digits and underscores (the `\b` notion).
fn words(line: &str) -> impl Iterator<Item = &str> {
    line.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|w| !w.is_empty())
}

/// R-1. The scanned product text names no agent vendor (case-insensitive, whole word). The
/// adapter files are exempt.
#[test]
fn product_text_names_no_agent_vendor() {
    let vendors = vendor_words();
    let mut hits = Vec::new();
    for path in product_text_files() {
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        for (n, line) in text.lines().enumerate() {
            for w in words(line) {
                if vendors.contains(&w.to_lowercase()) {
                    let rel = path.strip_prefix(repo_root()).unwrap_or(&path);
                    hits.push(format!("{}:{}: {}", rel.display(), n + 1, w));
                }
            }
        }
    }
    assert!(
        hits.is_empty(),
        "agent vendor names in product text (use \"agent\"; see CONTRIBUTING.md):\n{}",
        hits.join("\n")
    );
}

/// R-2. CONTRIBUTING.md has the section "Wording: agents and people" with its two key
/// sentences: say "agent" and "the person" or "you", and name a framework only in adapter files.
#[test]
fn contributing_states_agent_neutral_rule() {
    let text = read("CONTRIBUTING.md");
    let start = text
        .find("\n## Wording: agents and people\n")
        .unwrap_or_else(|| {
            panic!("CONTRIBUTING.md has no `## Wording: agents and people` section")
        });
    let body = &text[start + 1..];
    let end = body[1..].find("\n## ").map_or(body.len(), |i| i + 1);
    let section = body[..end].to_lowercase();
    assert!(
        section.contains("say **agent**") && section.contains("the person"),
        "section lacks the sentence: say agent, and the person or you"
    );
    assert!(
        section.contains("adapter files"),
        "section lacks the sentence: name a framework only in its adapter files"
    );
}
