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
    Scan(ScanArgs),
    Settings,
}

#[derive(Args, Debug)]
pub struct ScanArgs {
    #[arg(short, long, required = false)]
    pub input_directory: Option<PathBuf>,

    #[arg(short, long, required = false)]
    pub output_directory: Option<PathBuf>,

    #[arg(short, long, required = false)]
    pub settings: Option<PathBuf>,

    #[arg(short, long, default_value = "false")]
    pub bin_others: bool,
}
