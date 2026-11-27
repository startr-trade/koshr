//! The release workflow checks CI first and publishes nothing in a manual run without `publish` (FEAT-007).

use koshr_archtest::read;
use std::collections::BTreeMap;

struct Job {
    needs: Vec<String>,
    body: String,
}

/// The jobs of release.yml: name -> (`needs`, raw text), read by indentation (no YAML dependency).
fn jobs(text: &str) -> BTreeMap<String, Job> {
    let mut out: BTreeMap<String, Job> = BTreeMap::new();
    let mut current: Option<String> = None;
    let mut in_jobs = false;
    for line in text.lines() {
        if line.starts_with("jobs:") {
            in_jobs = true;
            continue;
        }
        if !in_jobs {
            continue;
        }
        if line.starts_with("  ") && !line.starts_with("   ") && line.trim_end().ends_with(':') {
            let name = line.trim().trim_end_matches(':').to_string();
            out.insert(
                name.clone(),
                Job {
                    needs: vec![],
                    body: String::new(),
                },
            );
            current = Some(name);
            continue;
        }
        if let Some(job) = current.as_ref().and_then(|n| out.get_mut(n)) {
            job.body.push_str(line);
            job.body.push('\n');
            if let Some(v) = line.strip_prefix("    needs:") {
                job.needs = v
                    .trim()
                    .trim_matches(['[', ']'])
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }
    out
}

fn reaches(jobs: &BTreeMap<String, Job>, from: &str, target: &str) -> bool {
    let mut stack = vec![from.to_string()];
    let mut seen = vec![];
    while let Some(j) = stack.pop() {
        if seen.contains(&j) {
            continue;
        }
        seen.push(j.clone());
        for n in jobs.get(&j).map(|j| j.needs.clone()).unwrap_or_default() {
            if n == target {
                return true;
            }
            stack.push(n);
        }
    }
    false
}
/// Every job other than `ci-green` waits (directly or through another job) for `ci-green`, and the
/// two jobs that publish run only on a tag push or when the `publish` input is set.
#[test]
fn release_workflow_checks_ci_first() {
    let text = read(".github/workflows/release.yml");
    let jobs = jobs(&text);
    assert!(jobs.contains_key("ci-green"), "no ci-green job");
    for name in jobs.keys().filter(|n| *n != "ci-green") {
        assert!(
            reaches(&jobs, name, "ci-green"),
            "job `{name}` does not need ci-green"
        );
    }
    let mut publishing = 0;
    for (name, job) in &jobs {
        let publishes = job.body.contains("cargo publish -p koshr --locked\n")
            || job.body.contains("gh release create");
        if publishes {
            publishing += 1;
            let cond = job
                .body
                .lines()
                .find(|l| l.starts_with("    if:"))
                .unwrap_or_else(|| panic!("publishing job `{name}` has no job-level if"));
            assert!(
                cond.contains("github.event_name == 'push'") && cond.contains("inputs.publish"),
                "job `{name}` is not gated on a tag push or inputs.publish: {cond}"
            );
        } else {
            assert!(
                !job.body.contains("    if:") || !job.body.contains("inputs.publish"),
                "job `{name}` does not publish but is gated on inputs.publish"
            );
        }
    }
    assert_eq!(
        publishing, 2,
        "expected the cargo publish and gh release create jobs"
    );
    assert!(
        jobs.get("publish-crates")
            .expect("no publish-crates job")
            .body
            .contains("cargo publish -p koshr --locked --dry-run"),
        "publish-crates does not run cargo publish --dry-run first"
    );
}

/// `scan` runs Trivy over the archives and the SBOM (HIGH,CRITICAL, fixable only, failing the job),
/// runs in a manual run too, and both publishing jobs come after it.
#[test]
fn release_workflow_scans_artifacts_before_publish() {
    let text = read(".github/workflows/release.yml");
    let jobs = jobs(&text);
    let scan = jobs.get("scan").expect("no scan job");
    assert!(scan.body.contains("aquasecurity/trivy-action@"));
    assert!(scan.body.contains("scan-type: fs"), "no fs scan");
    assert!(scan.body.contains("scan-type: sbom"), "no sbom scan");
    assert!(scan.body.contains("severity: HIGH,CRITICAL"));
    assert!(scan.body.contains("ignore-unfixed: true"));
    assert!(scan.body.contains("exit-code: \"1\""));
    assert!(
        !scan.body.contains("publish") && !scan.body.contains("    if:"),
        "scan must run in a manual run too"
    );
    for name in ["publish-crates", "github-release"] {
        assert!(
            reaches(&jobs, name, "scan"),
            "{name} does not come after scan"
        );
    }
}

/// `provenance` attests build provenance for the archives, and only it may mint tokens.
#[test]
fn release_attests_build_provenance() {
    let text = read(".github/workflows/release.yml");
    let jobs = jobs(&text);
    let provenance = jobs.get("provenance").expect("no provenance job");
    assert!(
        provenance.body.contains("actions/attest-build-provenance@"),
        "provenance has no attest-build-provenance step"
    );
    assert!(
        provenance.body.contains("artifacts/*.tar.gz")
            && provenance.body.contains("artifacts/*.zip")
    );
    assert!(
        provenance.body.contains("id-token: write")
            && provenance.body.contains("attestations: write")
    );
    for (name, job) in &jobs {
        if name != "provenance" {
            assert!(
                !job.body.contains("id-token: write") && !job.body.contains("attestations: write"),
                "job `{name}` must not have id-token or attestations write"
            );
        }
    }
    let top = text.split("\njobs:").next().expect("header");
    assert!(
        !top.contains("id-token: write"),
        "top-level permissions grant id-token"
    );
    assert!(
        reaches(&jobs, "publish-crates", "provenance"),
        "publish-crates does not come after provenance"
    );
}

/// Only the two publishing jobs have `contents: write` or the registry token.
#[test]
fn only_publishing_jobs_hold_write_access() {
    let text = read(".github/workflows/release.yml");
    for (name, job) in &jobs(&text) {
        if name != "publish-crates" {
            assert!(
                !job.body.contains("CARGO_REGISTRY_TOKEN"),
                "job `{name}` has the registry token"
            );
        }
        if name != "github-release" {
            assert!(
                !job.body.contains("contents: write"),
                "job `{name}` has contents: write"
            );
        }
    }
}

/// cargo-cyclonedx picks the package by `--manifest-path`; it has no `-p`.
#[test]
fn sbom_step_selects_the_crate_by_manifest_path() {
    let text = read(".github/workflows/release.yml");
    let line = text
        .lines()
        .find(|l| l.trim_start().starts_with("cargo cyclonedx"))
        .expect("release.yml runs cargo cyclonedx");
    let args: Vec<&str> = line.split_whitespace().collect();
    assert!(
        args.windows(2)
            .any(|w| w == ["--manifest-path", "crates/koshr/Cargo.toml"]),
        "the sbom step must select the crate with --manifest-path crates/koshr/Cargo.toml: {line}"
    );
    assert!(
        !args
            .iter()
            .any(|a| *a == "-p" || a.starts_with("--package")),
        "cargo cyclonedx has no -p/--package option: {line}"
    );
}

/// kanbanr writes the notes to `docs/guide/releases/v<version>.md`: the release step reads that path
/// from the tag name as is, and a step that also runs without publishing checks the file exists.
/// (The files themselves are not checked here: the user guide arrives with another item.)
#[test]
fn release_notes_path_matches_kanbanr() {
    let text = read(".github/workflows/release.yml");
    assert!(
        text.contains("--notes-file \"docs/guide/releases/${GITHUB_REF_NAME}.md\""),
        "gh release create must read docs/guide/releases/${{GITHUB_REF_NAME}}.md"
    );
    assert!(
        text.contains("test -f \"docs/guide/releases/v${version}.md\""),
        "a manual-run step must check docs/guide/releases/v${{version}}.md exists"
    );
}
