use anyhow::{anyhow, Result};
use std::{fs, process::Command};

const SVC: &str = "/etc/systemd/system/ugb.service";
const TMR: &str = "/etc/systemd/system/ugb.timer";

pub fn install_units() -> Result<()> {
    // oneshot service
    fs::write(SVC, r#"[Unit]
Description=UFW Geo Block sync

[Service]
Type=oneshot
ExecStart=/usr/local/bin/ugb sync
"#)?;

    // timer: twice a day
    fs::write(TMR, r#"[Unit]
Description=Run ugb sync periodically

[Timer]
OnBootSec=5min
OnUnitActiveSec=12h
Persistent=true

[Install]
WantedBy=timers.target
"#)?;

    run("systemctl", &["daemon-reload"])?;
    run("systemctl", &["enable", "--now", "ugb.timer"])?;
    Ok(())
}

pub fn uninstall_units() -> Result<()> {
    let _ = run("systemctl", &["disable", "--now", "ugb.timer"]);
    let _ = fs::remove_file(TMR);
    let _ = fs::remove_file(SVC);
    run("systemctl", &["daemon-reload"])?;
    Ok(())
}

fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let out = Command::new(cmd).args(args).output()?;
    if !out.status.success() {
        return Err(anyhow!("{} {:?} failed: {}", cmd, args, String::from_utf8_lossy(&out.stderr)));
    }
    Ok(())
}

