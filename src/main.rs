use std::{fs, io};
use std::io::Read;
use std::path::Path;

use reqwest::blocking::Response;
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

trait ToLocalPath {
    fn to_local_path(self: &Self) -> Option<String>;
}

impl ToLocalPath for String {
    fn to_local_path(self: &Self) -> Option<String> {
        if self.is_empty() {
            ()
        }

        let path = Path::new(self);

        let (stem, extension) = (path.file_stem()?.to_str()?, path.extension()?.to_str()?);

        let concat = format!("{}.{}", stem, extension);

        return Some(concat)
    }
}

fn download_library(&lib: &Library) {
    let directory = lib.directory.to_local_path().expect("Missing library directory.");
    let directory_path = Path::new(&directory);

    if !directory_path.exists() {
        fs::create_dir_all(directory_path).expect(&*format!("Failed to create directories {}", &directory))
    }

    let download_url = lib.download;

    let concat = format!("{}/{}", directory, download_url.to_local_path().expect("Missing download url."));
    let download_path = Path::new(&concat);

    if download_path.exists() {
        return;
    }


}

#[tokio::main]
async fn main() {
    let path = &*fs::read_to_string(Path::new("settings.json")).expect("Unable to find json file.");

    let settings: Settings = serde_json::from_str(path).expect("Failed to read json file");

    let mut url = settings.jvm.download;

    let _path = url.to_local_path();

    let libraries = settings.libraries;

    for library in &libraries {
        download_library(&library);
    }
}