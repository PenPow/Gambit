mod stockfish;

use crate::engine::known::stockfish::build_stockfish;
use crate::engine::{Cleanup, EngineHandle};
use anyhow::Context;
use std::path::{Path, PathBuf};
use std::process::Command;

struct KnownEngine {
    repo_url: &'static str,
    default_ref: &'static str,
    build: fn(&Path) -> anyhow::Result<PathBuf>,
}

fn known_engines() -> &'static [(&'static str, KnownEngine)] {
    &[(
        "stockfish",
        KnownEngine {
            repo_url: "https://github.com/official-stockfish/Stockfish.git",
            default_ref: "master",
            build: build_stockfish,
        },
    )]
}

pub fn build_named_engine(name_spec: &str) -> anyhow::Result<EngineHandle> {
    let (name, pinned_ref) = match name_spec.split_once('@') {
        Some((n, r)) => (n, Some(r)),
        None => (name_spec, None),
    };

    let Some((_, known)) = known_engines().iter().find(|(n, _)| *n == name) else {
        let available: Vec<&str> = known_engines().iter().map(|(n, _)| *n).collect();
        anyhow::bail!(
            "unknown engine `{name}`; known engines are: {}",
            available.join(", ")
        );
    };
    let git_ref = pinned_ref.unwrap_or(known.default_ref);

    let workdir = tempfile::Builder::new()
        .prefix(&format!("gambit-compare-{name}-"))
        .tempdir()
        .context("creating temp dir for engine checkout")?;

    let status = Command::new("git")
        .args(["clone", "--depth", "1", "--branch", git_ref, known.repo_url])
        .arg(workdir.path())
        .output()
        .with_context(|| format!("cloning `{name}` at ref `{git_ref}`"))?
        .status;

    if !status.success() {
        anyhow::bail!("git clone failed for `{name}` at ref `{git_ref}` (exit: {status})");
    }

    let executable = (known.build)(workdir.path())
        .with_context(|| format!("building `{name}` at ref `{git_ref}`"))?;

    Ok(EngineHandle {
        identity: format!("{name}@{git_ref}"),
        executable,
        _cleanup: Cleanup::TempDir(workdir),
    })
}
