use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fs, path::Path};

const STATE_PATH: &str = "/var/lib/ugb/state.json";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub enabled: BTreeSet<String>,
}

pub fn load() -> Result<State> {
    if !Path::new(STATE_PATH).exists() {
        return Ok(State::default());
    }
    let s = fs::read_to_string(STATE_PATH)?;
    Ok(serde_json::from_str(&s)?)
}

pub fn save(st: &State) -> Result<()> {
    if let Some(dir) = Path::new(STATE_PATH).parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = format!("{STATE_PATH}.tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(st)?)?;
    fs::rename(tmp, STATE_PATH)?;
    Ok(())
}

impl State {
    pub fn enable(&mut self, region: &str) -> Result<()> {
        let r = normalize(region);
        self.enabled.insert(r);
        Ok(())
    }
    pub fn disable(&mut self, region: &str) -> Result<()> {
        let r = normalize(region);
        if !self.enabled.remove(&r) {
            return Err(anyhow!("Region not enabled: {}", region));
        }
        Ok(())
    }
    pub fn print(&self) {
        for r in &self.enabled {
            println!("{r}");
        }
    }
}

fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

