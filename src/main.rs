use std::{fs};
use std::path::Path;
use std::process::Command;

use bytes::Bytes;
use serde::Deserialize;

mod unzip_jre;
mod concurrent;

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
pub struct Library {
    url: Url,
    directory: Url
}

// fn request_blocking(url: &Url) -> reqwest::Result<Bytes> {
//     reqwest::blocking::get(url)?.bytes()
// }

async fn request_non_blocking(url: &Url) -> Result<Bytes, reqwest::Error> {
    reqwest::get(url).await?.bytes().await
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

#[tokio::main]
async fn main() {
    let json_path = &*fs::read_to_string(Path::new("settings.json")).unwrap();

    let settings: Settings = serde_json::from_str(json_path).unwrap();

    let libraries = settings.libraries;

    concurrent::create_libraries(libraries).await;

    let jvm = settings.jvm;

    let output = create_command()
        .args([
            &format!("{}\\bin\\java.exe", unzip_jre::really_slow_unzip(jvm.url))[..],
            "-cp",
            "libs/*",
            &jvm.main_class[..]
        ])
        .output()
        .unwrap();

    println!("{}", String::from_utf8_lossy(&*output.stderr));
}