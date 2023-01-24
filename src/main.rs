use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    libraries: Vec<Library>
}

#[derive(Deserialize)]
struct Library {
    url: String
}

#[tokio::main]
async fn main() {
    let str = fs::read_to_string(Path::new("launcher.json")).unwrap();

    let settings: Settings = serde_json::from_str(&*str).unwrap();

    println!("{}", settings.libraries[0].url);
}
