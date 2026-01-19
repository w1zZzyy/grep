use crate::lib::buffer::Buffer;
use crossbeam::channel::{SendError, Sender};
use std::{fs::OpenOptions, io, path::Path};

pub struct Producer {
    sender: Sender<Buffer>,
}

#[derive(Debug)]
pub enum ProducerError {
    IO(io::Error),
    Other(String),
    Send(SendError<Buffer>),
}

impl Producer {
    pub fn new(tx: &Sender<Buffer>) -> Self {
        Producer {
            sender: tx.clone(),
        }
    }

    pub fn run(&self, path: &Path) -> Result<(), ProducerError> {
        if path.is_dir() {
            let entries = path.read_dir()?; // io::Error -> ProducerError::IO
            for entry in entries {
                let entry_path = entry?.path();
                self.run(&entry_path)?; // рекурсивно обходим
            }
            Ok(())
        } else if path.is_file() {
            self.handle_file(path.to_str())
        } else {
            let err = format!("path {} is not a file or directory", path.as_os_str().display());
            Err(ProducerError::Other(err))
        }
    }

    fn handle_file(&self, path_opt: Option<&str>) -> Result<(), ProducerError> {
        if let Some(path) = path_opt {
            let mut file = OpenOptions::new().read(true).open(path)?;
            loop {
                let mut buffer = Buffer::new();
                if let Ok(bytes) = buffer.read_file(&mut file, path) {
                     if bytes == 0 {
                        break;
                    }

                    self.sender.send(buffer)?;
                }
            }
            Ok(())
        } else {
            Err(ProducerError::Other("path error".into()))
        }
    }
}

impl From<io::Error> for ProducerError {
    fn from(err: io::Error) -> Self {
        ProducerError::IO(err)
    }
}
impl From<String> for ProducerError {
    fn from(err: String) -> Self {
        ProducerError::Other(err)
    }
}
impl From<SendError<Buffer>> for ProducerError {
    fn from(err: SendError<Buffer>) -> Self {
        ProducerError::Send(err)
    }
}
