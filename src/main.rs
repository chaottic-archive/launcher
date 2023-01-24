use std::fs;
use std::io::Read;
use std::path::Path;

use bytes::Bytes;
use serde::Deserialize;

type Url = String;

#[derive(Deserialize)]
struct Settings {
    jvm: Jvm,
    libraries: Vec<Library>
}

#[derive(Deserialize)]
struct Jvm {
    download: Url,
    main_class: Url,
    arguments: Vec<String>
}

#[derive(Deserialize)]
struct Library {
    download: Url,
    directory: Url
}

trait Localize {
    fn localize(self: &Self) -> Option<String>;
}

impl Localize for String {
    fn localize(self: &Self) -> Option<String> {
        if self.is_empty() {
            ()
        }

        let path = Path::new(self);

        let (stem, extension) = (path.file_stem()?.to_str()?, path.extension()?.to_str()?);

        Some(format!("{}.{}", stem, extension))
    }
}

fn get_content(url: &Url) -> reqwest::Result<Bytes> {
    reqwest::blocking::get(url)?.bytes()
}

fn download_library(library: &Library) {
    let download = &library.download;

    let concat = format!("{}/{}", library.directory, &download.localize().expect("Missing library download url"));

    let path = Path::new(&concat);
    if path.exists() {
        return;
    }

    let content = get_content(&download).expect("");

    fs::write(path, content).expect("Failed to write downloaded library")
}

fn main() {
    let json_file_path = &*fs::read_to_string(Path::new("settings.json")).expect("Missing json file");

    let settings: Settings = serde_json::from_str(json_file_path).expect("Failed to read json file");

    let libraries = settings.libraries;

    for library in &libraries {
        download_library(&library);
    }
}