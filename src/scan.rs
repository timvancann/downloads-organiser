use crate::cli::ScanArgs;
use crate::settings::Settings;
use crate::{get_default_path, settings, Stats};
use std::fs::read_dir;
use std::path::PathBuf;

pub fn scan_cli(args: ScanArgs) -> crate::prelude::Result<()> {
    let input = args.input_directory.unwrap_or_else(|| get_default_path());
    let output = args.output_directory.unwrap_or_else(|| get_default_path());
    let settings = args
        .settings
        .map(|s| settings::deserialize_settings(s))
        .unwrap_or_else(|| Ok(settings::default_settings()))?;
    let bin_others = args.bin_others;

    println!("Input directory: {:?}", input);
    println!("Output directory: {:?}", output);

    let mut stats = Stats { total_files: 0 };

    for entry in read_dir(&input)? {
        let entry = entry?;
        let path = entry.path();
        create_folder_if_not_exists(&output)?;
        let result = process_file(path, &settings);
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

enum FileResult {
    File(PathBuf, String),
    Other(PathBuf, String),
    None,
}

fn process_file(input: PathBuf, settings: &Settings) -> FileResult {
    if input.is_file() {
        match input.extension() {
            Some(ext) => {
                let extension = ext.to_str().unwrap();
                for ext in settings.extensions.iter() {
                    if ext.extensions.contains(&extension.to_string()) {
                        return FileResult::File(input, ext.path.to_string());
                    }
                }
                FileResult::Other(input, settings.other_dir.to_string())
            }
            None => FileResult::None,
        }
    } else if is_app(&input) {
        FileResult::File(input, settings.app_dir.to_string())
    } else {
        FileResult::None
    }
}

fn move_file(path: &PathBuf, stats: &mut Stats, folder: PathBuf) -> crate::prelude::Result<()> {
    let _ = create_folder_if_not_exists(&folder)?;
    let _ = move_file_to_folder(&path, &folder)?;
    stats.total_files += 1;
    Ok(())
}

fn is_dotfile(path: &PathBuf) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    file_name.starts_with(".")
}

fn create_folder_if_not_exists(folder_name: &PathBuf) -> crate::prelude::Result<()> {
    if !folder_name.exists() {
        std::fs::create_dir(folder_name)?;
    }
    Ok(())
}

fn move_file_to_folder(path: &PathBuf, folder_name: &PathBuf) -> crate::prelude::Result<()> {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let new_path = folder_name.join(file_name);
    std::fs::rename(path, new_path)?;
    Ok(())
}

fn is_app(path: &PathBuf) -> bool {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .ends_with(".app")
}
