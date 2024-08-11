use crate::cli::ScanArgs;
use crate::settings::Settings;
use crate::{get_default_path, settings, Stats};
use anyhow::Result;
use std::fs::{create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("failed to read file {0}")]
    InputFile(PathBuf),
    #[error("Failed to move file {from} to {to}")]
    Move { from: PathBuf, to: PathBuf },
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Directory {0} error")]
    Directory(PathBuf),
}

pub fn scan_cli(args: ScanArgs) -> Result<()> {
    let input = args.input_directory.unwrap_or_else(get_default_path);
    let output = args.output_directory.unwrap_or_else(get_default_path);
    let settings = args
        .settings
        .map(settings::deserialize_settings)
        .unwrap_or_else(settings::default_settings);
    let bin_others = args.bin_others;

    let mut stats = Stats { total_files: 0 };

    for entry in read_dir(&input)? {
        let input_file = entry?.path();

        let result = process_file(input_file, &settings);
        match result {
            FileResult::File(path, folder) => {
                move_file(&path, &mut stats, output.join(folder))?;
            }
            FileResult::Other(path, folder) => {
                if bin_others {
                    move_file(&path, &mut stats, output.join(folder))?;
                }
            }
            _ => {}
        }
    }

    println!("Total files moved: {}", stats.total_files);
    Ok(())
}

pub fn scan_others(args: ScanArgs) -> Result<()> {
    let settings = args
        .settings
        .map(settings::deserialize_settings)
        .unwrap_or_else(settings::default_settings);

    scan_cli(ScanArgs {
        input_directory: Some(
            args.input_directory
                .unwrap_or_else(get_default_path)
                .join(&settings.other_dir),
        ),
        output_directory: args.output_directory,
        settings: None,
        bin_others: false,
    })
}

enum FileResult {
    File(PathBuf, String),
    Other(PathBuf, String),
    None,
}

fn process_file(input: PathBuf, settings: &Settings) -> FileResult {
    if input.is_file() {
        match input.extension().and_then(|ext| ext.to_str()) {
            Some(extension) => {
                for ext in settings.extensions.iter() {
                    if ext.extensions.contains(&extension.to_string()) {
                        return FileResult::File(input, ext.path.to_string());
                    }
                }
                FileResult::Other(input, settings.other_dir.to_string())
            }
            None => FileResult::None,
        }
    } else if let Some(true) = is_app(&input) {
        FileResult::File(input, settings.app_dir.to_string())
    } else {
        FileResult::None
    }
}

fn move_file(path: &PathBuf, stats: &mut Stats, folder: PathBuf) -> Result<()> {
    create_dir_all(&folder)?;
    move_file_to_folder(path, &folder)?;
    stats.total_files += 1;
    Ok(())
}

fn move_file_to_folder(path: &PathBuf, folder_name: &Path) -> Result<(), ScanError> {
    let new_path = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .map(|name| folder_name.join(name));

    if let Some(new_path) = new_path {
        match std::fs::rename(path, &new_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(ScanError::Move {
                from: path.clone(),
                to: new_path,
            }),
        }
    } else {
        Err(ScanError::InputFile(path.clone()))
    }
}

fn is_app(path: &Path) -> Option<bool> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.ends_with(".app"))
}
