use crate::prelude::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub fn settings_cli() -> Result<()> {
    let settings = default_settings();
    serialize_settings(&settings)?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub extensions: Vec<Extension>,
    pub other_dir: String,
    pub app_dir: String,
}

#[derive(Deserialize, Serialize)]
pub struct Extension {
    pub extensions: Vec<String>,
    pub path: String,
}
pub fn default_settings() -> Settings {
    Settings {
        extensions: vec![
            Extension {
                extensions: vec![
                    String::from("mp3"),
                    String::from("wav"),
                    String::from("flac"),
                    String::from("ogg"),
                ],
                path: String::from("_audio"),
            },
            Extension {
                extensions: vec![
                    String::from("mp4"),
                    String::from("mkv"),
                    String::from("avi"),
                    String::from("flv"),
                ],
                path: String::from("_video"),
            },
            Extension {
                extensions: vec![
                    String::from("zip"),
                    String::from("tar"),
                    String::from("gz"),
                    String::from("rar"),
                    String::from("dmg"),
                ],
                path: String::from("_archive"),
            },
            Extension {
                extensions: vec![
                    String::from("jpg"),
                    String::from("jpeg"),
                    String::from("png"),
                    String::from("gif"),
                    String::from("webp"),
                ],
                path: String::from("_image"),
            },
            Extension {
                extensions: vec![
                    String::from("pdf"),
                    String::from("doc"),
                    String::from("docx"),
                    String::from("xls"),
                    String::from("xlsx"),
                    String::from("yaml"),
                    String::from("yml"),
                ],
                path: String::from("_document"),
            },
            Extension {
                extensions: vec![String::from("exe"), String::from("msi")],
                path: String::from("_installer"),
            },
            Extension {
                extensions: vec![String::from("app")],
                path: String::from("_app"),
            },
        ],
        other_dir: String::from("_other"),
        app_dir: String::from("_app"),
    }
}

fn serialize_settings(extensions: &Settings) -> Result<()> {
    let file = std::fs::File::create("settings.json")?;
    serde_json::to_writer(file, extensions)?;
    Ok(())
}

pub fn deserialize_settings(path: PathBuf) -> Result<Settings> {
    let file = std::fs::File::open(path)?;
    let extensions = serde_json::from_reader(file)?;
    Ok(extensions)
}
