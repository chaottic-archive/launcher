use std::{fs, io, process};
use std::io::Write;
use std::path::Path;
use std::process::Command;

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

#[cfg(target_os = "windows")]
fn create_command() -> Command {
    let mut command = Command::new("cmd");
    command.arg("/C");
    command
}

#[cfg(target_os = "linux")]
fn create_command(jre_path: String, main_class: String) -> Command {
    let mut command = Command::new("sh");
    command.arg("-c");
    command
}

fn main() {
    let json_path = &*fs::read_to_string(Path::new("settings.json")).expect("Missing json file");

    let settings: Settings = serde_json::from_str(json_path).expect("Failed to read json file");

    let libraries = &settings.libraries;

    for library in libraries {
        download_library(&library);
    }

    let jvm = settings.jvm;

    let output = create_command()
        .args([
            &format!("{}\\bin\\java.exe", unzip_jre::really_slow_unzip(jvm.url))[..],
            "-cp",
            "libs/*",
            &jvm.main_class[..]
        ])
        .output()
        .expect("Failed to run");

    println!("{}", String::from_utf8_lossy(&*output.stderr));
}