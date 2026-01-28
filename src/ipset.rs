use anyhow::{anyhow, Result};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn apply_sets(v4: &[String], v6: &[String]) -> Result<()> {
    // ensure sets exist
    run("ipset", &["create", "ugb-geo", "hash:net", "-exist"])?;
    run("ipset", &["create", "ugb-geo6", "hash:net", "family", "inet6", "-exist"])?;

    // rebuild via ipset restore (fast)
    ipset_restore("ugb-geo", v4, false)?;
    ipset_restore("ugb-geo6", v6, true)?;
    Ok(())
}

fn ipset_restore(name: &str, cidrs: &[String], inet6: bool) -> Result<()> {
    let mut script = String::new();
    script.push_str(&format!("flush {name}\n"));
    for c in cidrs {
        if c.trim().is_empty() { continue; }
        // ipset accepts v6 in inet6 set; no extra validation here
        script.push_str(&format!("add {name} {c}\n"));
    }

    let mut child = Command::new("ipset")
        .args(["restore"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().ok_or_else(|| anyhow!("no stdin"))?.write_all(script.as_bytes())?;
    let out = child.wait_with_output()?;
    if !out.status.success() {
        return Err(anyhow!("ipset restore failed: {}", String::from_utf8_lossy(&out.stderr)));
    }
    Ok(())
}

fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let out = Command::new(cmd).args(args).output()?;
    if !out.status.success() {
        return Err(anyhow!("{} {:?} failed: {}", cmd, args, String::from_utf8_lossy(&out.stderr)));
    }
    Ok(())
}

