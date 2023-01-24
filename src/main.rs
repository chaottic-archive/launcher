use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    libraries: Libraries
}

#[derive(Deserialize)]
struct Libraries {
    directory: String,
    downloads: Vec<Download>
}

#[derive(Deserialize)]
struct Download {
    url: String
}

#[tokio::main]
async fn main() {
    let file = &*fs::read_to_string(Path::new("settings.json")).unwrap();

    let settings: Settings = serde_json::from_str(file).unwrap();

    let libraries = settings.libraries;

    let directory = Path::new(&libraries.directory);
    if !directory.exists() {
        fs::create_dir_all(directory).expect("Failed to create directories.");
    }

    for download in &libraries.downloads {
        if download.url.is_empty() {
            continue
        }

        let url = Path::new(&download.url);

        let stem = url.file_stem().unwrap().to_str().unwrap();
        let extension = url.extension().unwrap().to_str().unwrap();

        let formatted = format!("{}/{}.{}", directory.to_str().unwrap(), stem, extension);

        let path = Path::new(&formatted);
        if path.exists() {
            continue;
        }

        let response = reqwest::get(&download.url).await.unwrap();

        let bytes = response.bytes().await.unwrap();

        fs::write(path, &bytes).expect("Failed to write.");
    }
}

