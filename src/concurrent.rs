use std::fs;
use std::io::Write;
use std::path::Path;

use futures::{stream, StreamExt};

use crate::{Library, request_non_blocking, Url};

enum Err {

}

async fn from_url(directory: &Url, url: &Url) -> Option<String> {
    let path = Path::new(&url);

    let (stem, extension) = (path.file_stem()?.to_str()?, path.extension()?.to_str()?);

    Some(format!("{}/{}.{}", directory, stem, extension))
}

async fn create_library(library: &Library) -> Result<(), reqwest::Error> {
    let url = &library.url;

    let buffer = request_non_blocking(url).await?;

    // let from_url = from_url(&library.directory, url).await.ok_or())?;

    let path = Path::new(&from_url);

    if let Some(p) = path.parent() {
        if !p.exists() {
            fs::create_dir_all(p).ok()?;
        }
    }

    fs::write(path, buffer).ok()?;

    Ok(())
}

fn filter_library(library: &Library) -> Option<bool> {
    let url = &library.url;

    if url.is_empty() { return None; }

    let from_url = from_url(&library.directory, &url)?;

    let path = Path::new(from_url);

    if path.exists() { return None; }

    Some(false)
}

pub async fn create_libraries(libraries: Vec<Library>) {
    let stream = stream::iter(libraries.iter().filter(|&library| {filter_library(library).unwrap()}).map(create_library));

    // stream.for_each_concurrent(1, |f| async move { f.await.unwrap(); }).await;
}

async fn create_temporary(url: Url) {
    let mut temp = tempfile::NamedTempFile::new().unwrap();

    let buffer = request_non_blocking(&url).await.unwrap();

    temp.write_all(&*buffer).unwrap();
}

pub async fn create_jre() -> String {

    String::from("")
}