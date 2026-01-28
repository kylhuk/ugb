use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeoDb {
    pub version: u64,
    pub generatedAt: String,
    pub continents: Vec<Continent>,
}
#[derive(Debug, Deserialize)]
pub struct Continent {
    pub name: String,
    pub countries: Vec<Country>,
}
#[derive(Debug, Deserialize)]
pub struct Country {
    pub name: String,
    pub iso2: String,
    pub cidrs4: Vec<String>,
    pub cidrs6: Vec<String>,
}

pub fn fetch_default_db() -> Result<GeoDb> {
    // Replace with your GitHub release asset URL later.
    let url = "https://example.com/ugb/geo.json";
    let body = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    Ok(serde_json::from_str(&body)?)
}

impl GeoDb {
    pub fn resolve_enabled(&self, enabled: &std::collections::BTreeSet<String>) -> Result<(Vec<String>, Vec<String>)> {
        let mut v4 = Vec::new();
        let mut v6 = Vec::new();

        for item in enabled {
            // match continent
            if let Some(c) = self.continents.iter().find(|c| norm(&c.name) == *item) {
                for country in &c.countries {
                    v4.extend(country.cidrs4.iter().cloned());
                    v6.extend(country.cidrs6.iter().cloned());
                }
                continue;
            }
            // match country by name or ISO2
            let mut found = false;
            for cont in &self.continents {
                for country in &cont.countries {
                    if norm(&country.name) == *item || norm(&country.iso2) == *item {
                        v4.extend(country.cidrs4.iter().cloned());
                        v6.extend(country.cidrs6.iter().cloned());
                        found = true;
                    }
                }
            }
            if !found {
                return Err(anyhow!("Unknown region/country: {}", item));
            }
        }

        Ok((v4, v6))
    }
}

fn norm(s: &str) -> String { s.trim().to_lowercase() }

