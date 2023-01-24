use std::fs;
use std::path::Path;

use futures::{stream, StreamExt};
use futures::future::MaybeDone::Future;
use futures::stream::FuturesUnordered;

use crate::{Library, request_non_blocking, Url};

fn to_local(directory: &Url, url: &Url) -> Option<String> {
    let path = Path::new(&url);

    let (stem, extension) = (
        path.file_stem()?.to_str()?,
        path.extension()?.to_str()?
    );

    Some(format!("{}/{}.{}", directory, stem, extension))
}

async fn create_library(library: &Library) -> () {
    let buffer = request_non_blocking(&library.url).await.unwrap();

    let local = to_local(&library.directory, &library.url).unwrap();

    let path = Path::new(&local);

    if let Some(p) = path.parent() {
        if !p.exists() {
            fs::create_dir_all(p).unwrap();
        }
    }

    fs::write(path, buffer).unwrap();
}

pub async fn create_libraries(libraries: Vec<Library>) {
    let stream = stream::iter(libraries.iter().filter(|&library| {
        let url = &library.url;
        if url.is_empty() {
            return false
        }

        let local = to_local(&library.directory, url).unwrap();

        let path = Path::new(&local);

        if path.exists() {
            return false
        }

        true
    }).map(create_library));

    stream.for_each_concurrent(1, |f| async move {f.await}).await;
}


pub async fn create_jre() {

}