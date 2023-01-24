use std::{fs, io};
use std::future::Future;
use std::io::Write;
use std::path::Path;
use futures::{stream, StreamExt, TryStreamExt};

use crate::{download_blocking, Url};

pub fn slow_unzip(url: Url) {
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();

    let buffer = download_blocking(&url).unwrap();

    temp_file.write_all(&*buffer).unwrap();

    let mut archive = zip::ZipArchive::new(temp_file).unwrap();

    use futures::stream::TryStreamExt;

    let stream = stream::iter(archive.file_names()).try_for_each_concurrent(4, |n| async move {

    });
}