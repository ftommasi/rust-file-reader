use chrono::DateTime;
use std::fs::{self, DirEntry, File};
#[derive(Debug)]
pub struct FileData {
    filename: String,
    filesize: u64,
    is_dir: bool,
    filedate: String, //TODO this should probably be a datetime
    filetime: String, //TODO this should probably be a datetime
}

fn systemtime_strftime<T>(dt: T, format: &str) -> String
where
    T: Into<time::OffsetDateTime>,
{
    dt.into().format(format)
}

pub fn process_directory(dir: &DirEntry) -> FileData {
    let datetime = dir
        .metadata()
        .unwrap()
        .accessed()
        .unwrap()
        .format("%d-%m-%y");
    let fileday = dir.metadata().unwrap().accessed().unwrap();
    let filetime = dir.metadata().unwrap().accessed().unwrap();
    let cur_dir_data = FileData {
        filename: String::from(dir.path().to_str().unwrap()),
        filesize: dir.metadata().unwrap().len(),
        is_dir: dir.path().is_dir(),
        filedate: String::from("TODAY"),
        filetime: String::from("NOW"),
    };
    cur_dir_data
}

fn main() {
    let path_or_err = fs::read_dir("/home/ftommasi/tdrive-portal");
    match (path_or_err) {
        Ok(path) => {
            for file in path {
                //TODO: recurse through sub-dirs and get all file data
                if file.is_ok() {
                    let dir = file.unwrap();
                    let cur_dir_data = process_directory(&dir);
                    println!("processed {:?}", cur_dir_data);
                }
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
