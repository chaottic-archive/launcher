use std::fs;
use std::io::{Write};
use std::path::Path;

use futures::{stream, StreamExt};
use log::info;

use crate::{get_bytes, Library, Url};

// TODO:: Remove unneeded 'unwrap'

fn url_from_library(library: &Library) -> Option<Url> {
    let url = &library.url;

    let path = Path::new(url);

    let (stem, extension) = (path.file_stem()?.to_str()?, path.extension()?.to_str()?);

    Some(format!("{}/{}.{}", library.directory, stem, extension))
}

async fn create_library(library: &Library) -> Result<(), reqwest::Error> {
    let bytes = get_bytes(&library.url).await?;

    let url = url_from_library(library).unwrap();

    let path = Path::new(&url);

    if let Some(p) = path.parent() {
        if !p.exists() {
            fs::create_dir_all(p).unwrap();
        }
    }

    fs::write(path, bytes).unwrap();

    Ok(())
}

fn doesnt_have_library(library: &Library) -> Option<()> {
    if library.url.is_empty() { return None; }

    let url = url_from_library(&library)?;

    let path = Path::new(&url);

    if path.exists() { return None; }

    info!("Downloading {}", path.to_str()?);

    Some(())
}

pub(crate) async fn create_libraries(libraries: Vec<Library>) {
    let stream = stream::iter(libraries.iter().filter(|&library| {doesnt_have_library(library).is_some()}).map(create_library));

    stream.for_each_concurrent(2, |f| async move { f.await.unwrap(); }).await;
}

async fn create_archive(url: Url) -> Option<()> {
    let mut temp = tempfile::NamedTempFile::new().ok()?;

    let buffer = get_bytes(&url).await.unwrap();

    temp.write_all(&*buffer).ok()?;

    zip::ZipArchive::new(temp).ok()?.extract(Path::new(".")).ok()?;

    Some(())
}

pub(crate) async fn create_jre(url: Url, directory: &Url) {
    let path = Path::new(&directory);

    if path.exists() {
        return;
    }

    info!("Downloading the JRE");

    create_archive(url).await.unwrap();
}