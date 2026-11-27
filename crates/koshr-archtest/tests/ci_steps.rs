//! `make ci` and the CI workflows agree, and every action is pinned (FEAT-006).

use koshr_archtest::{read, repo_root};
use std::process::Command;

/// The targets `$(MAKE) … <target>` in the `ci:` recipe, in order, without the conditional `no-c`.
fn make_ci_targets() -> Vec<String> {
    let makefile = read("Makefile");
    let mut lines = makefile.lines().skip_while(|l| !l.starts_with("ci:"));
    lines.next();
    lines
        .take_while(|l| l.starts_with('\t'))
        .filter_map(|l| {
            let t = l.trim_start_matches(['\t', '@']);
            let rest = t.strip_prefix("$(MAKE)")?;
            let target = rest.split_whitespace().last()?;
            Some(target.to_string())
        })
        .collect()
}

/// The `make <target>` run steps of the job `job` in ci.yml, in order.
fn workflow_make_steps(text: &str, job: &str) -> Vec<String> {
    let head = format!("  {job}:");
    let mut in_job = false;
    let mut out = Vec::new();
    for l in text.lines() {
        if l == head {
            in_job = true;
            continue;
        }
        if in_job {
            let top = l.starts_with("  ") && !l.starts_with("   ") && !l.trim().is_empty();
            if top || (!l.starts_with(' ') && !l.is_empty()) {
                break;
            }
            if let Some(t) = l.trim().strip_prefix("run: make ") {
                out.push(t.trim().to_string());
            }
        }
    }
    out
}

/// R-1. The ordered `make ci` steps equal the ordered `run: make <target>` steps of ci.yml
/// (`lint` then `test`), except `actionlint`, which `lint` runs in CI and `make lint` does not need.
#[test]
fn make_ci_matches_ci_workflow_steps() {
    let ci = read(".github/workflows/ci.yml");
    let mut steps = workflow_make_steps(&ci, "lint");
    steps.extend(workflow_make_steps(&ci, "test"));
    steps.retain(|s| s != "actionlint");
    assert_eq!(
        make_ci_targets(),
        steps,
        "`make ci` and ci.yml run different steps"
    );
}

/// R-3. Every `uses:` that is not local is pinned to a full commit SHA, with a version comment.
#[test]
fn actions_pinned_to_sha_and_runners_pinned() {
    let dir = repo_root().join(".github/workflows");
    let mut seen = 0;
    for e in std::fs::read_dir(&dir).expect("workflows dir") {
        let p = e.expect("entry").path();
        if p.extension().is_none_or(|x| x != "yml") {
            continue;
        }
        let name = p.file_name().expect("name").to_string_lossy().into_owned();
        let text = std::fs::read_to_string(&p).expect("workflow");
        for l in text.lines() {
            let t = l.trim().trim_start_matches("- ");
            let Some(v) = t.strip_prefix("uses:") else {
                continue;
            };
            let v = v.trim();
            if v.starts_with("./") {
                continue;
            }
            seen += 1;
            let (spec, comment) = match v.split_once(" # ") {
                Some((s, c)) => (s, Some(c)),
                None => (v, None),
            };
            let ok = spec.split_once('@').is_some_and(|(repo, sha)| {
                !repo.is_empty()
                    && repo.contains('/')
                    && repo
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || "._/-".contains(c))
                    && sha.len() == 40
                    && sha.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
            });
            assert!(ok, "{name}: `uses: {v}` is not pinned to a 40-hex SHA");
            if let Some(c) = comment {
                assert!(c.starts_with('v'), "{name}: version comment `{c}` lacks v");
            }
        }
    }
    assert!(seen > 0, "no `uses:` found in the workflows");

    let runners = workflow_runners();
    assert!(
        !runners.is_empty(),
        "no runner images found in the workflows"
    );
    for (name, image) in runners {
        assert!(
            runner_pinned(&image),
            "{name}: runner `{image}` must end in a version, never -latest"
        );
    }
}

/// A runner image name is pinned when its last `-` segment starts with a digit (`ubuntu-24.04`).
fn runner_pinned(image: &str) -> bool {
    !image.contains("latest")
        && image
            .rsplit_once('-')
            .is_some_and(|(_, v)| v.starts_with(|c: char| c.is_ascii_digit()))
}

/// Every runner image a workflow names: `runs-on` values and matrix `os` entries; a
/// `${{ … }}` reference is skipped because the matrix entries it points at are read directly.
fn runner_images(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for l in text.lines() {
        let t = l.trim().trim_start_matches("- ");
        let Some(value) = t
            .strip_prefix("runs-on:")
            .or_else(|| t.strip_prefix("os:"))
            .map(str::trim)
        else {
            continue;
        };
        if value.starts_with("${{") {
            continue;
        }
        out.extend(
            value
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty()),
        );
    }
    out
}

fn workflow_runners() -> Vec<(String, String)> {
    let dir = repo_root().join(".github/workflows");
    let mut all = Vec::new();
    for e in std::fs::read_dir(&dir).expect("workflows dir") {
        let p = e.expect("entry").path();
        if p.extension().is_none_or(|x| x != "yml") {
            continue;
        }
        let name = p.file_name().expect("name").to_string_lossy().into_owned();
        let text = std::fs::read_to_string(&p).expect("workflow");
        all.extend(runner_images(&text).into_iter().map(|i| (name.clone(), i)));
    }
    all
}

/// The runner check itself: it accepts versioned images and rejects `-latest` ones.
#[test]
fn runner_check_rejects_latest_images() {
    assert!(runner_pinned("ubuntu-24.04") && runner_pinned("macos-15"));
    assert!(!runner_pinned("ubuntu-latest") && !runner_pinned("ubuntu"));
    let text = "    runs-on: ${{ matrix.os }}\n        os: [a-1, \"b-2\"]\n    runs-on: c-3\n    runs-on: d-latest";
    assert_eq!(runner_images(text), ["a-1", "b-2", "c-3", "d-latest"]);
}

/// R-3. actionlint passes over the workflows. Install: download the linux_amd64 tarball of the
/// latest release from github.com/rhysd/actionlint, verify it against the release's checksums
/// file, and put `actionlint` on PATH (or set `$ACTIONLINT`).
#[test]
fn actionlint_passes_over_workflows() {
    let bin = std::env::var("ACTIONLINT").unwrap_or_else(|_| "actionlint".into());
    let out = Command::new(&bin)
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|e| {
            panic!(
                "cannot run `{bin}`: {e}. Install it from the rhysd/actionlint release tarball \
                 (verify the checksums file) into ~/.local/bin, or set ACTIONLINT"
            )
        });
    assert!(
        out.status.success(),
        "actionlint failed:\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// R-6. codeql.yml is one job `sast`, run on `release/**` pushes and on demand, whose analyze
/// step has `category: sast`.
#[test]
fn codeql_runs_as_sast_on_release_branches() {
    let text = read(".github/workflows/codeql.yml");
    let lines: Vec<&str> = text.lines().map(str::trim).collect();
    for needle in [
        "sast:",
        "workflow_dispatch:",
        "branches: [\"release/**\"]",
        "category: sast",
        "languages: rust",
    ] {
        assert!(lines.contains(&needle), "codeql.yml lacks `{needle}`");
    }
    let jobs = text.lines().skip_while(|l| *l != "jobs:").skip(1);
    let names: Vec<&str> = jobs
        .filter(|l| l.starts_with("  ") && !l.starts_with("   ") && l.trim_end().ends_with(':'))
        .collect();
    assert_eq!(
        names,
        ["  sast:"],
        "codeql.yml must hold exactly the job sast"
    );
}

/// DEF-001 R-1. The test job installs the tools its archtests run (actionlint, cargo-deny) before
/// `make test`: cargo-deny through install-action, actionlint through its pinned download script.
#[test]
fn ci_test_job_installs_actionlint_and_cargo_deny() {
    let text = read(".github/workflows/ci.yml");
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|l| *l == "  test:")
        .expect("ci.yml has a test job");
    let end = lines[start + 1..]
        .iter()
        .position(|l| l.starts_with("  ") && !l.starts_with("   ") && l.trim_end().ends_with(':'))
        .map_or(lines.len(), |i| start + 1 + i);
    let job = &lines[start..end];
    let make_test = job
        .iter()
        .position(|l| l.trim() == "run: make test")
        .expect("the test job runs make test");
    let before = &job[..make_test];
    let deny = before.iter().any(|l| {
        let t = l.trim();
        t.starts_with("tool:") && t.contains("cargo-deny")
    });
    // install-action cannot install actionlint (run 45000700624): it must come from the
    // commit-pinned download script, as in the lint job, and never from install-action.
    let actionlint_by_install_action = before.iter().any(|l| {
        let t = l.trim();
        t.starts_with("tool:") && t.contains("actionlint")
    });
    let actionlint_by_script = before
        .iter()
        .any(|l| l.contains("scripts/download-actionlint.bash"));
    assert!(
        deny,
        "ci.yml's test job must install cargo-deny before `make test`"
    );
    assert!(
        actionlint_by_script && !actionlint_by_install_action,
        "ci.yml's test job must install actionlint with the pinned download script before `make test`"
    );
}
