use serde::{Deserialize, Serialize};
use std::fs::read_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    name: String,
    path: String,
}

#[tauri::command]
pub fn list_files(path: String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    let paths = read_dir(path).unwrap();

    for path in paths {
        let p = path.unwrap();
        files.push(File {
            name: p.file_name().to_str().unwrap().to_string(),
            path: p.path().to_str().unwrap().to_string(),
        })
    }

    files
}
