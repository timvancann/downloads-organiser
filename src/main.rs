mod cli;
mod scan;
mod settings;

use crate::cli::{Commands, CLI};
use crate::scan::scan_cli;
use crate::settings::settings_cli;
use anyhow::Result;
use clap::Parser;
use directories::UserDirs;
use std::path::PathBuf;

fn get_default_path() -> PathBuf {
    UserDirs::new()
        .unwrap()
        .download_dir()
        .unwrap()
        .to_path_buf()
}

struct Stats {
    total_files: usize,
}

fn main() -> Result<()> {
    let args = CLI::parse();

    match args.command {
        Commands::Scan(args) => scan_cli(args),
        Commands::Settings => settings_cli(),
    }
}
