use std::{fs, io};
use std::future::Future;
use std::io::Write;
use std::path::Path;
use futures::{stream, StreamExt, TryStreamExt};

use crate::{Url};

//pub fn slow_unzip(url: Url) {
//    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
//
//    let buffer = request_blocking(&url).unwrap();
//
//    temp_file.write_all(&*buffer).unwrap();
//
//    let mut archive = zip::ZipArchive::new(temp_file).unwrap();
//
//    use futures::stream::TryStreamExt;
//
//    //let stream = stream::iter(archive.file_names()).try_for_each_concurrent(4, |n| async move {
//    //
//    //});
//}

pub fn really_slow_unzip(url: Url) -> String {
//    let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create a temporary file");
//
//    let content = request_blocking(&url).expect("Failed to get jvm archive");
//
//    temp_file.write_all(&*content).expect("Failed to write to temporary file");
//
//    let mut archive = zip::ZipArchive::new(temp_file).expect("Failed to create zip archive.");
//
//    for i in 0..archive.len() {
//        let mut file = archive.by_index(i).expect("Failed to get archive entry");
//
//        let path = file.enclosed_name().to_owned().expect("");
//        if path.to_str().and_then(|str| str.chars().last()).unwrap() == '/' {
//            continue
//        }
//
//        if let Some(p) = path.parent() {
//            if !p.exists() {
//                fs::create_dir_all(p).expect("")
//            }
//        }
//
//        fs::File::create(path).and_then(|mut writer| io::copy(&mut file, &mut writer)).expect("");
//    }

    String::from("jdk-17.0.6+10-jre")
}