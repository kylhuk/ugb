use anyhow::Result;
use clap::Parser;

mod cli;
mod db;
mod ipset;
mod state;
mod systemd;
mod ufw;

use cli::{Cli, Cmd};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Add { region, apply } => {
            let mut st = state::load()?;
            st.enable(&region)?;
            state::save(&st)?;
            if apply {
                return ugb_sync();
            }
            Ok(())
        }
        Cmd::Remove { region, apply } => {
            let mut st = state::load()?;
            st.disable(&region)?;
            state::save(&st)?;
            if apply {
                return ugb_sync();
            }
            Ok(())
        }
        Cmd::List => {
            let st = state::load()?;
            st.print();
            Ok(())
        }
        Cmd::Sync => ugb_sync(),
        Cmd::ServiceInstall => systemd::install_units(),
        Cmd::ServiceUninstall => systemd::uninstall_units(),
        Cmd::Completion { shell } => {
            cli::print_completion(shell);
            Ok(())
        }
    }
}

fn ugb_sync() -> Result<()> {
    let st = state::load()?;
    let (repo_url, repo_branch) = config::repo();
    let repo_dir = std::path::Path::new("/var/lib/ugb/repo");

    repo::sync_repo(&repo_url, &repo_branch, repo_dir)?;

    let geo = db::load_embedded_geo()?; // Dhall->JSON, contains iso2 + grouping only
    let wanted_iso2 = geo.resolve_enabled_iso2(&st.enabled)?;

    let v4_tar = repo_dir.join("data/ipdeny/all-zones.tar.gz");
    let v6_tar = repo_dir.join("data/ipdeny/ipv6-all-zones.tar.gz");

    let v4 = ipdeny::cidrs_from_tar_gz(&v4_tar, &wanted_iso2)?;
    let v6 = ipdeny::cidrs_from_tar_gz(&v6_tar, &wanted_iso2)?;

    ipset::apply_sets(&v4, &v6)?;
    ufw::ensure_hook_installed()?;
    ufw::reload()?;
    Ok(())
}
