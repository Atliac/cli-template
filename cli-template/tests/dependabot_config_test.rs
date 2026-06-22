/// Integration tests for .github/dependabot.yml configuration.
///
/// These tests validate the structure and values of the Dependabot configuration,
/// particularly that update schedules are set to "monthly" as changed in this PR.
use std::fs;
use std::path::PathBuf;

fn dependabot_yml_path() -> PathBuf {
    // CARGO_MANIFEST_DIR points to cli-template/; go up one level to the workspace root
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .join("..")
        .join(".github")
        .join("dependabot.yml")
}

fn read_dependabot_yml() -> String {
    let path = dependabot_yml_path();
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read dependabot.yml at {path:?}: {e}"))
}

// ---------------------------------------------------------------------------
// File-level sanity checks
// ---------------------------------------------------------------------------

#[test]
fn dependabot_yml_exists() {
    assert!(
        dependabot_yml_path().exists(),
        "Expected .github/dependabot.yml to exist"
    );
}

#[test]
fn dependabot_yml_is_not_empty() {
    let contents = read_dependabot_yml();
    assert!(
        !contents.trim().is_empty(),
        "dependabot.yml must not be empty"
    );
}

#[test]
fn dependabot_yml_version_is_2() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains("version: 2"),
        "dependabot.yml must declare 'version: 2', got:\n{contents}"
    );
}

// ---------------------------------------------------------------------------
// Ecosystem presence
// ---------------------------------------------------------------------------

#[test]
fn github_actions_ecosystem_is_configured() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"package-ecosystem: "github-actions""#),
        "dependabot.yml must configure the github-actions package ecosystem"
    );
}

#[test]
fn cargo_ecosystem_is_configured() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"package-ecosystem: "cargo""#),
        "dependabot.yml must configure the cargo package ecosystem"
    );
}

// ---------------------------------------------------------------------------
// Schedule interval — the values changed in this PR ("weekly" → "monthly")
// ---------------------------------------------------------------------------

/// Both schedule blocks must use "monthly".  We count occurrences to make
/// sure both the github-actions and cargo entries are covered.
#[test]
fn both_ecosystems_use_monthly_interval() {
    let contents = read_dependabot_yml();
    let monthly_count = contents
        .lines()
        .filter(|line| {
            // Match the actual schedule interval line (not comments)
            let trimmed = line.trim();
            trimmed.starts_with("interval:") && trimmed.contains(r#""monthly""#)
        })
        .count();
    assert_eq!(
        monthly_count, 2,
        "Expected exactly 2 'interval: \"monthly\"' schedule entries (one per ecosystem), \
         found {monthly_count}"
    );
}

/// Regression guard: neither ecosystem should still be set to "weekly".
#[test]
fn no_ecosystem_uses_weekly_interval() {
    let contents = read_dependabot_yml();
    let weekly_interval_lines: Vec<&str> = contents
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            // Only flag actual interval values, not comments that mention "weekly"
            trimmed.starts_with("interval:") && trimmed.contains(r#""weekly""#)
        })
        .collect();
    assert!(
        weekly_interval_lines.is_empty(),
        "Found 'interval: \"weekly\"' in dependabot.yml — expected 'monthly' after the PR change. \
         Offending lines: {weekly_interval_lines:?}"
    );
}

/// Regression guard: neither ecosystem should use "daily" either.
#[test]
fn no_ecosystem_uses_daily_interval() {
    let contents = read_dependabot_yml();
    let daily_interval_lines: Vec<&str> = contents
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("interval:") && trimmed.contains(r#""daily""#)
        })
        .collect();
    assert!(
        daily_interval_lines.is_empty(),
        "Found 'interval: \"daily\"' in dependabot.yml — expected 'monthly'. \
         Offending lines: {daily_interval_lines:?}"
    );
}

// ---------------------------------------------------------------------------
// github-actions specific settings
// ---------------------------------------------------------------------------

#[test]
fn github_actions_directory_is_root() {
    let contents = read_dependabot_yml();
    // The github-actions block appears before the cargo block; verify root dir is set
    // by checking that `directory: "/"` appears at least once (it appears in both blocks)
    assert!(
        contents.contains(r#"directory: "/""#),
        "dependabot.yml must set directory to \"/\" for at least one ecosystem"
    );
}

#[test]
fn github_actions_has_open_pull_requests_limit_3() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains("open-pull-requests-limit: 3"),
        "github-actions block must have open-pull-requests-limit: 3"
    );
}

#[test]
fn github_actions_has_ci_label() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"- "ci""#),
        "github-actions block must include the 'ci' label"
    );
}

#[test]
fn github_actions_has_dependencies_label() {
    let contents = read_dependabot_yml();
    // "dependencies" label appears in both blocks
    assert!(
        contents.contains(r#"- "dependencies""#),
        "dependabot.yml must include a 'dependencies' label"
    );
}

#[test]
fn github_actions_groups_wildcard_pattern() {
    let contents = read_dependabot_yml();
    // The github-actions group uses a wildcard to group all actions together
    assert!(
        contents.contains("- \"*\"") || contents.contains("- '*'"),
        "dependabot.yml must have a wildcard group pattern for github-actions"
    );
}

// ---------------------------------------------------------------------------
// Cargo specific settings
// ---------------------------------------------------------------------------

#[test]
fn cargo_has_rust_label() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"- "rust""#),
        "cargo block must include the 'rust' label"
    );
}

#[test]
fn cargo_commit_message_prefix_is_chore() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"prefix: "chore""#),
        "cargo commit-message must use prefix 'chore'"
    );
}

#[test]
fn cargo_commit_message_includes_scope() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"include: "scope""#),
        "cargo commit-message must include 'scope' (for conventional commits)"
    );
}

#[test]
fn cargo_has_tokio_ecosystem_group() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains("tokio-ecosystem:"),
        "cargo groups must contain a 'tokio-ecosystem' group"
    );
}

#[test]
fn cargo_tokio_group_includes_tokio_pattern() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"- "tokio*""#),
        "cargo tokio-ecosystem group must include the 'tokio*' pattern"
    );
}

#[test]
fn cargo_tokio_group_includes_axum_pattern() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"- "axum*""#),
        "cargo tokio-ecosystem group must include the 'axum*' pattern"
    );
}

#[test]
fn cargo_has_minor_patch_group() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains("cargo-minor-patch:"),
        "cargo groups must contain a 'cargo-minor-patch' group"
    );
}

#[test]
fn cargo_minor_patch_group_covers_minor_updates() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"- "minor""#),
        "cargo-minor-patch group must include 'minor' in update-types"
    );
}

#[test]
fn cargo_minor_patch_group_covers_patch_updates() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"- "patch""#),
        "cargo-minor-patch group must include 'patch' in update-types"
    );
}

// ---------------------------------------------------------------------------
// Boundary / negative tests
// ---------------------------------------------------------------------------

/// Ensure the comment block listing valid interval options is present,
/// confirming valid options are documented inline.
#[test]
fn interval_options_comment_is_present() {
    let contents = read_dependabot_yml();
    assert!(
        contents.contains(r#"# Options: "daily", "weekly", "monthly""#),
        "dependabot.yml should document interval options in a comment"
    );
}

/// Verify that neither ecosystem omits the directory key entirely.
#[test]
fn both_ecosystems_specify_directory() {
    let contents = read_dependabot_yml();
    let directory_count = contents
        .lines()
        .filter(|line| line.trim().starts_with("directory:"))
        .count();
    assert_eq!(
        directory_count, 2,
        "Expected exactly 2 'directory:' keys (one per ecosystem), found {directory_count}"
    );
}

/// Confirm exactly two ecosystems are configured (no accidental additions/deletions).
#[test]
fn exactly_two_package_ecosystems_are_configured() {
    let contents = read_dependabot_yml();
    let ecosystem_count = contents
        .lines()
        .filter(|line| line.trim().starts_with("- package-ecosystem:"))
        .count();
    assert_eq!(
        ecosystem_count, 2,
        "Expected exactly 2 package-ecosystem entries, found {ecosystem_count}"
    );
}
