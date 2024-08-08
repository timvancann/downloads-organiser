use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan a directory and move files based on their extension
    Scan(ScanArgs),
    /// Generate a default settings file
    Settings,
}

#[derive(Args, Debug)]
pub struct ScanArgs {
    /// The directory to scan, defaults to the user's download directory
    #[arg(short, long, required = false)]
    pub input_directory: Option<PathBuf>,

    /// The directory to move files to, defaults to the user's download directory
    #[arg(short, long, required = false)]
    pub output_directory: Option<PathBuf>,

    /// The settings file to use, defaults to the default build-in settings
    #[arg(short, long, required = false)]
    pub settings: Option<PathBuf>,

    /// Whether to bin all other unmatched files into a separate folder
    #[arg(short, long, default_value = "false")]
    pub bin_others: bool,
}
