use anyhow::{anyhow, Result};
use std::{path::Path, process::Command};

pub fn sync_repo(url: &str, branch: &str, dir: &Path) -> Result<()> {
    if !dir.join(".git").exists() {
        run("git", &[
            "clone", "--depth", "1", "--branch", branch, url,
            dir.to_str().ok_or_else(|| anyhow!("bad dir"))?,
        ])?;
        return Ok(());
    }

    run_in(dir, "git", &["fetch", "--depth", "1", "origin", branch])?;
    run_in(dir, "git", &["reset", "--hard", &format!("origin/{}", branch)])?;
    Ok(())
}

fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let out = Command::new(cmd).args(args).output()?;
    if !out.status.success() {
        return Err(anyhow!("{} {:?} failed: {}", cmd, args, String::from_utf8_lossy(&out.stderr)));
    }
    Ok(())
}

fn run_in(dir: &Path, cmd: &str, args: &[&str]) -> Result<()> {
    let out = Command::new(cmd).current_dir(dir).args(args).output()?;
    if !out.status.success() {
        return Err(anyhow!("{} {:?} failed: {}", cmd, args, String::from_utf8_lossy(&out.stderr)));
    }
    Ok(())
}

