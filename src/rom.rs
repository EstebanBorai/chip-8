use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Rom(Vec<u8>);

impl Rom {
    pub fn from_path(path: &PathBuf) -> Self {
        let file = fs::read(path).unwrap();

        Rom(file)
    }

    pub fn write(path: &PathBuf, bytes: Vec<u8>) {
        let mut file = fs::File::create(path).unwrap();

        file.write_all(&bytes).unwrap();
    }

    pub fn bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<Vec<u8>> for Rom {
    fn from(bytes: Vec<u8>) -> Self {
        Rom(bytes)
    }
}
