use crate::engine::{Cleanup, EngineHandle, WorktreeGuard};
use anyhow::Context;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn current_repo_root() -> anyhow::Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("running `git rev-parse --show-toplevel`")?;

    if !output.status.success() {
        anyhow::bail!("not inside a git repository");
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
    ))
}

fn resolve_gitref(repo_root: &Path, gitref: &str) -> anyhow::Result<String> {
    if Command::new("git")
        .current_dir(repo_root)
        .args(["rev-parse", "--verify", gitref])
        .output()?
        .status
        .success()
    {
        let sha = Command::new("git")
            .current_dir(repo_root)
            .args(["rev-parse", gitref])
            .output()?
            .stdout;

        return Ok(String::from_utf8_lossy(&sha).trim().to_string());
    }

    for remote in ["origin", "upstream"] {
        let fetch_status = Command::new("git")
            .current_dir(repo_root)
            .args([
                "fetch",
                remote,
                &format!("refs/heads/{gitref}:refs/remotes/{remote}/{gitref}"),
            ])
            .output()?
            .status;

        if fetch_status.success() {
            let remote_ref = format!("{remote}/{gitref}");

            let output = Command::new("git")
                .current_dir(repo_root)
                .args(["rev-parse", "--verify", &remote_ref])
                .output()?;

            if output.status.success() {
                return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
    }

    anyhow::bail!("Could not resolve git ref: {gitref}")
}

pub fn build_own_ref(
    repo_root: &Path,
    gitref: &str,
    binary_name: &str,
) -> anyhow::Result<EngineHandle> {
    let checkout_dir = tempfile::Builder::new()
        .prefix("gambit-compare-worktree-")
        .tempdir()
        .context("creating temp dir for git worktree")?
        .keep();

    let resolved = resolve_gitref(repo_root, gitref)?;

    let status = Command::new("git")
        .current_dir(repo_root)
        .args(["worktree", "add", "--detach"])
        .arg(&checkout_dir)
        .arg(&resolved)
        .output()
        .with_context(|| format!("running `git worktree add` for ref `{resolved}`"))?
        .status;

    if !status.success() {
        anyhow::bail!("git worktree add failed for ref `{resolved}` (exit: {status})");
    }

    let guard = WorktreeGuard {
        repo_root: repo_root.to_path_buf(),
        checkout_dir: checkout_dir.clone(),
    };

    let build_status = Command::new("cargo")
        .current_dir(&checkout_dir)
        .args(["build", "--release", "--bin", binary_name])
        .output()
        .context("running `cargo build --release`")?
        .status;

    if !build_status.success() {
        anyhow::bail!("cargo build --release failed for ref `{gitref}`");
    }

    let executable = checkout_dir
        .join("target")
        .join("release")
        .join(binary_name);
    if !executable.exists() {
        anyhow::bail!(
            "build succeeded but expected binary was not found at {}",
            executable.display()
        );
    }

    Ok(EngineHandle {
        identity: format!("{binary_name}@{gitref}"),
        executable,
        _cleanup: Cleanup::Worktree(guard),
    })
}
