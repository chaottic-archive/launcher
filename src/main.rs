use std::{fs, io};
use std::io::Write;
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
    url: Url,
    main_class: Url,
    arguments: Vec<String>
}

#[derive(Deserialize)]
struct Library {
    url: Url,
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

fn download_and_run_jvm(settings: &Settings) {
    let url = &settings.jvm.url;

    let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create a temporary file");

    let content = get_content(url).expect("Failed to get jvm archive");

    temp_file.write_all(&*content).expect("Failed to write to temporary file");

    let mut archive = zip::ZipArchive::new(temp_file).expect("Failed to create zip archive.");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to get archive entry");

        let path = file.enclosed_name().to_owned().expect("");
        if path.to_str().and_then(|str| str.chars().last()).unwrap() == '/' {
            continue
        }

        if let Some(p) = path.parent() {
            if !p.exists() {
                fs::create_dir_all(p).expect("")
            }
        }

        fs::File::create(path).and_then(|mut writer| io::copy(&mut file, &mut writer)).expect("");
    }
}

fn get_content(url: &Url) -> reqwest::Result<Bytes> {
    reqwest::blocking::get(url)?.bytes()
}

fn download_library(library: &Library) {
    let directory = &library.directory;
    let mut path = Path::new(directory);

    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create directories");
    }

    let url = &library.url;

    let concat = format!("{}/{}", library.directory, &url.localize().expect("Missing library download url"));

    path = Path::new(&concat);
    if path.exists() {
        return;
    }
    let content = get_content(&url).expect("");

    fs::write(path, content).expect("Failed to write downloaded library")
}

fn main() {
    let json_file_path = &*fs::read_to_string(Path::new("settings.json")).expect("Missing json file");

    let settings: Settings = serde_json::from_str(json_file_path).expect("Failed to read json file");

    let libraries = &settings.libraries;

    for library in libraries {
        download_library(&library);
    }

    download_and_run_jvm(&settings);
}