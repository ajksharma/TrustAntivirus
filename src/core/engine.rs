use io::SystemFile;
use std::fs::{File, Metadata};
use std::path::Path;
use std::process;
use std::time::SystemTime;
use walkdir::{Iter, WalkDir};

///
///
///
pub struct Engine<'a> {

    path: &'a Path

}

impl<'a> Engine<'a> {

    ///
    ///
    ///
    pub fn new() -> Engine<'a> {
        Engine {
            path: Path::new("") // Defaults to the root directory of the current user's main disk
        }
    }

    ///
    ///
    ///
    pub fn from(path: &str) -> Engine {
        Engine {
            path: Path::new(path)
        }
    }

    ///
    ///
    ///
    pub fn do_scan(&self) {
        let cur_time: SystemTime = SystemTime::now();

        if !self.path.exists() {
            println!("Directory or file does not exist.");

            process::exit(1);
        }

        let mut result: Option<ScanResult> = None;
        if self.path.is_dir() {
            result = Engine::scan_dir(self.path);
        } else {
            result = Engine::scan_file(self.path);
        }

        println!("Total size: {}", SystemFile::human_readable_size(result.unwrap().total_scan_size));
        println!("Finished in {} seconds", SystemTime::now().duration_since(cur_time).unwrap().as_secs());
    }

    ///
    ///
    ///
    pub(self) fn scan_dir(dir: &Path) -> Option<ScanResult> {
        println!("Scanning dir: {}", dir.to_str().unwrap());

        let mut total_size: f64 = 0.0;
        let dir_iter: Iter = WalkDir::new(dir).into_iter();

        for entry in dir_iter.filter_map(|e| e.ok()) {
            let file: File = File::open(entry.path()).unwrap();
            let metadata: Metadata = file.metadata().unwrap();
            let file_path: String = String::from(entry.path().to_str().unwrap());
            let sys_file: SystemFile = SystemFile::from(file_path);

            total_size += metadata.len() as f64;

            println!("{}", sys_file);
        }

        Some(
            ScanResult {
                total_scan_size: total_size
            }
        )
    }

    ///
    ///
    ///
    pub(self) fn scan_file(file_path: &Path) -> Option<ScanResult> {
        println!("Scanning file: {}", file_path.to_str().unwrap());

        let file: File = File::open(file_path).unwrap();
        let metadata: Metadata = file.metadata().unwrap();
        let file_path_as_str: String = String::from(file_path.to_str().unwrap());
        let sys_file: SystemFile = SystemFile::from(file_path_as_str);

        println!("{}", sys_file);

        Some(
            ScanResult {
                total_scan_size: metadata.len() as f64
            }
        )
    }

}

pub struct ScanResult {

    pub total_scan_size: f64

}

pub struct FileType {

    extension: String,
    mime_type: String

}

pub trait FileTypeScan {

    ///
    ///
    ///
    fn get_file_type() -> FileType;

}