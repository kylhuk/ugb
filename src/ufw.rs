use anyhow::{anyhow, Result};
use std::{fs, path::Path, process::Command};

const BEFORE4: &str = "/etc/ufw/before.rules";
const BEFORE6: &str = "/etc/ufw/before6.rules";

pub fn ensure_hook_installed() -> Result<()> {
    patch_file(BEFORE4, "ufw-before-input", "ugb-geo", false)?;
    patch_file(BEFORE6, "ufw6-before-input", "ugb-geo6", true)?;
    Ok(())
}

pub fn reload() -> Result<()> {
    let out = Command::new("ufw").arg("reload").output()?;
    if !out.status.success() {
        return Err(anyhow!(
            "ufw reload failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(())
}

fn patch_file(path: &str, chain: &str, setname: &str, _v6: bool) -> Result<()> {
    if !Path::new(path).exists() {
        return Err(anyhow!("Missing UFW file: {path}"));
    }
    let content = fs::read_to_string(path)?;

    let begin = "# BEGIN UGB";
    let end = "# END UGB";
    let block = format!("{begin}\n-A {chain} -m set --match-set {setname} src -j DROP\n{end}\n");

    let new_content = if content.contains(begin) && content.contains(end) {
        // replace existing block
        replace_block(&content, begin, end, &block)?
    } else {
        // insert right after the chain definition line if present, else append (safe but less ideal)
        insert_after_chain(&content, chain, &block)
    };

    if new_content != content {
        let tmp = format!("{path}.tmp");
        fs::write(&tmp, new_content)?;
        fs::rename(tmp, path)?;
    }
    Ok(())
}

fn replace_block(content: &str, begin: &str, end: &str, block: &str) -> Result<String> {
    let b = content
        .find(begin)
        .ok_or_else(|| anyhow!("begin not found"))?;
    let e = content.find(end).ok_or_else(|| anyhow!("end not found"))?;
    let e2 = e + end.len();
    Ok(format!("{}{}\n{}", &content[..b], block, &content[e2..]))
}

fn insert_after_chain(content: &str, chain: &str, block: &str) -> String {
    // UFW files usually contain a line like: :ufw-before-input - [0:0]
    let needle = format!(":{chain} ");
    if let Some(pos) = content.find(&needle) {
        // insert after the chain declaration line end
        let line_end = content[pos..]
            .find('\n')
            .map(|x| pos + x + 1)
            .unwrap_or(content.len());
        let mut out = String::new();
        out.push_str(&content[..line_end]);
        out.push_str(block);
        out.push_str(&content[line_end..]);
        out
    } else {
        // fallback: append (still works if chain exists later)
        format!("{content}\n{block}")
    }
}
