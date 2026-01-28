use anyhow::Result;
use flate2::read::GzDecoder;
use std::{collections::BTreeSet, fs::File, io::{BufRead, BufReader, Read}, path::Path};
use tar::Archive;

pub fn cidrs_from_tar_gz(tar_gz: &Path, wanted_iso2: &BTreeSet<String>) -> Result<Vec<String>> {
    let f = File::open(tar_gz)?;
    let gz = GzDecoder::new(f);
    let mut ar = Archive::new(gz);

    let mut out = Vec::new();

    for entry in ar.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if !name.ends_with(".zone") {
            continue;
        }

        // ipdeny files are like "cn.zone"
        let iso = name.trim_end_matches(".zone").to_lowercase();
        if !wanted_iso2.contains(&iso) {
            continue;
        }

        let mut buf = String::new();
        entry.read_to_string(&mut buf)?;
        for line in BufReader::new(buf.as_bytes()).lines() {
            let l = line?;
            let l = l.trim();
            if l.is_empty() { continue; }
            out.push(l.to_string());
        }
    }

    Ok(out)
}

