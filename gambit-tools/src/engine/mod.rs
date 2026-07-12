pub mod git;
pub mod known;

use crate::engine::git::build_own_ref;
use crate::engine::known::build_named_engine;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

#[derive(Debug, Clone)]
pub enum EngineSpec {
    OwnRef(String),
    Named(String),
}

pub fn parse_spec(s: &str) -> EngineSpec {
    match s.strip_prefix("external:") {
        Some(external) => EngineSpec::Named(external.to_string()),
        None => EngineSpec::OwnRef(s.to_string()),
    }
}

pub struct EngineHandle {
    pub identity: String,
    pub executable: PathBuf,
    _cleanup: Cleanup,
}

#[allow(dead_code)]
enum Cleanup {
    Worktree(WorktreeGuard),
    TempDir(TempDir),
}

struct WorktreeGuard {
    repo_root: PathBuf,
    checkout_dir: PathBuf,
}

impl Drop for WorktreeGuard {
    fn drop(&mut self) {
        let removed = Command::new("git")
            .current_dir(&self.repo_root)
            .args(["worktree", "remove", "--force"])
            .arg(&self.checkout_dir)
            .status();

        let cleaned = matches!(removed, Ok(status) if status.success());
        if !cleaned {
            let _ = std::fs::remove_dir_all(&self.checkout_dir);
            let _ = Command::new("git")
                .current_dir(&self.repo_root)
                .args(["worktree", "prune"])
                .status();
        }
    }
}

pub fn prepare_engine(
    spec: &EngineSpec,
    repo_root: &Path,
    own_binary_name: &str,
) -> anyhow::Result<EngineHandle> {
    match spec {
        EngineSpec::OwnRef(gitref) => build_own_ref(repo_root, gitref, own_binary_name),
        EngineSpec::Named(name_spec) => build_named_engine(name_spec),
    }
}

pub fn available_parallelism() -> String {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .to_string()
}
