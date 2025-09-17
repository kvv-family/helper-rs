
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Watermak {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilesCount {
    pub watermark: i32,
    pub inputs: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Progress {
    pub result_file: i32,
}