use crate::engine::available_parallelism;
use anyhow::Context;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn build_stockfish(checkout_dir: &Path) -> anyhow::Result<PathBuf> {
    let src_dir = checkout_dir.join("src");
    let jobs = available_parallelism();

    let arch_attempts = ["native", "x86-64-avx2", "x86-64"];
    let mut last_error = None;
    for arch in arch_attempts {
        let status = Command::new("make")
            .current_dir(&src_dir)
            .args(["-j", &jobs, "build", &format!("ARCH={arch}")])
            .output();

        match status {
            Ok(out) if out.status.success() => {
                let exe = src_dir.join("stockfish");
                if exe.exists() {
                    return Ok(exe);
                }

                last_error = Some(anyhow::anyhow!(
                    "`make` reported success for ARCH={arch} but no binary was produced"
                ));
            }
            Ok(out) => {
                last_error = Some(anyhow::anyhow!(format!(
                    "make exited with {} for ARCH={arch}",
                    out.status
                )));
            }
            Err(e) => {
                return Err(e).context("running `make`; is a C++ toolchain installed?");
            }
        }
    }
    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("failed to build stockfish")))
}
