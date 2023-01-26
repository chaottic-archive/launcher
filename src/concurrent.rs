use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;

use futures::{stream, StreamExt};

use crate::{get_bytes, Library, Url};

fn local_from_library(library: &Library) -> Option<Url> {
    local(&library.directory, &library.url)
}

fn local(directory: &Url, url: &Url) -> Option<Url> {
    let path = Path::new(&url);

    let (stem, extension) = (path.file_stem()?.to_str()?, path.extension()?.to_str()?);

    Some(format!("{}/{}.{}", directory, stem, extension))
}

// TODO:: Remove 'unwrap'
async fn create_library(library: &Library) -> Result<(), reqwest::Error> {
    let url = &library.url;

    let buffer = get_bytes(url).await?;

    let local = local_from_library(&library).unwrap();

    let path = Path::new(&local);

    if let Some(p) = path.parent() {
        if !p.exists() {
            fs::create_dir_all(p).unwrap();
        }
    }

    Ok(())
}

fn filter_library(library: &Library) -> Option<()> {
    let url = &library.url;

    if url.is_empty() { return None; }

    let local = local_from_library(&library)?;

    let path = Path::new(&*local);

    if path.exists() { return None; }

    Some(())
}

pub(crate) async fn create_libraries(libraries: Vec<Library>) {
    let stream = stream::iter(libraries.iter().filter(|&library| {filter_library(library).is_some()}).map(create_library));

    stream.for_each_concurrent(1, |f| async move { f.await.unwrap(); }).await;
}


// TODO::
async fn create_temporary(url: Url) {
    let mut temp = tempfile::NamedTempFile::new().unwrap();

    let buffer = get_bytes(&url).await.unwrap();

    temp.write_all(&*buffer).unwrap();
}

pub async fn create_jre() -> String {

    String::from("")
}