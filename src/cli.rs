use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::{generate, shells, Generator};
use std::io;

#[derive(Parser)]
#[command(name = "ugb", about = "UFW Geo Block", version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    Add {
        region: String,
        #[arg(long)]
        apply: bool,
    },
    Remove {
        region: String,
        #[arg(long)]
        apply: bool,
    },
    List,
    Sync,
    ServiceInstall,
    ServiceUninstall,
    Completion {
        shell: Shell,
    },
}

#[derive(Copy, Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

pub fn print_completion(shell: Shell) {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    match shell {
        Shell::Bash => generate(shells::Bash, &mut cmd, name, &mut io::stdout()),
        Shell::Zsh => generate(shells::Zsh, &mut cmd, name, &mut io::stdout()),
        Shell::Fish => generate(shells::Fish, &mut cmd, name, &mut io::stdout()),
    }
}
use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::{generate, shells, Generator};
use std::io;

#[derive(Parser)]
#[command(name = "ugb", about = "UFW Geo Block", version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    Add {
        region: String,
        #[arg(long)]
        apply: bool,
    },
    Remove {
        region: String,
        #[arg(long)]
        apply: bool,
    },
    List,
    Sync,
    ServiceInstall,
    ServiceUninstall,
    Completion {
        shell: Shell,
    },
}

#[derive(Copy, Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

pub fn print_completion(shell: Shell) {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    match shell {
        Shell::Bash => generate(shells::Bash, &mut cmd, name, &mut io::stdout()),
        Shell::Zsh => generate(shells::Zsh, &mut cmd, name, &mut io::stdout()),
        Shell::Fish => generate(shells::Fish, &mut cmd, name, &mut io::stdout()),
    }
}
