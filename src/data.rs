mod config;
mod user_data;

use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};
use std::fs::*;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::path::*;

pub use config::*;
pub use user_data::*;

fn read_file_to_vec(file: &mut File) -> Vec<u8> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    buf
}

fn get_file(path: &PathBuf) -> std::fs::File {
    std::fs::File::options()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .unwrap()
}
