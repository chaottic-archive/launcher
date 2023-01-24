use std::{fs, io};
use std::io::Write;
use std::path::Path;

use bytes::Bytes;
use serde::Deserialize;

mod unzip_jre;

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

fn download_blocking(url: &Url) -> reqwest::Result<Bytes> {
    reqwest::blocking::get(url)?.bytes()
}

// TODO:: Concurrent.
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

    let buffer = download_blocking(&url).unwrap();

    fs::write(path, buffer).expect("Failed to write downloaded library");
}

fn main() {
    let json_file_path = &*fs::read_to_string(Path::new("settings.json")).expect("Missing json file");

    let settings: Settings = serde_json::from_str(json_file_path).expect("Failed to read json file");

    let libraries = &settings.libraries;

    for library in libraries {
        download_library(&library);
    }

    unzip_jre::really_slow_unzip(settings.jvm.url);
}