use std::{fs::File, io::Read, string::FromUtf8Error};

pub struct Buffer {
    data: String,
    bytes: usize,
    path: Option<String>,
}

impl Buffer {
    pub fn new() -> Self {
        return Buffer {
            data: String::new(),
            bytes: 0,
            path: None,
        };
    }

    pub fn read_file(&mut self, file: &mut File, path: &str) -> Result<usize, std::io::Error> {
        self.bytes = file.read_to_string(&mut self.data)?;
        self.path = Some(path.to_string());
        Ok(self.bytes)
    }

    pub fn path(&self) -> Result<&str, String> {
        if let Some(res) = self.path.as_deref() {
            Ok(res)
        } else {
            Err("path was not settled".into())
        }
    }

    pub fn into_string(self) -> Result<String, FromUtf8Error> {
        Ok(self.data)
    }
}
