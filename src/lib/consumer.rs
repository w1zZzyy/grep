use crate::lib::{
    buffer::Buffer,
    results::{ResultValue, Results},
};
use crossbeam::channel::{Receiver, RecvError};
use std::{string::FromUtf8Error, sync::Arc};

#[derive(Clone)]
pub struct Consumer {
    receiver: Receiver<Buffer>,
    pattern: String,
    results: Arc<Results>,
}

#[derive(Debug)]
pub enum ConsumerError {
    Utf8(FromUtf8Error),
    Recv(RecvError),
    Other(String),
}

impl Consumer {
    pub fn new(
        rx: &Receiver<Buffer>,
        pattern: &str,
        res: Arc<Results>,
    ) -> Self {
        Consumer {
            receiver: (rx.clone()),
            pattern: pattern.into(),
            results: res,
        }
    }
    pub fn run(&self) -> Result<(), ConsumerError> {
        loop {
            if let Ok(buffer) = self.receiver.recv() {
                let path = buffer.path()?.to_string();
                let res = buffer
                    .into_string()?
                    .lines()
                    .enumerate()
                    .filter(|line| line.1.contains(&self.pattern))
                    .map(|item| item.into())
                    .collect::<Vec<ResultValue>>();
                self.results.push(&path, res);
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl From<FromUtf8Error> for ConsumerError {
    fn from(err: FromUtf8Error) -> Self {
        ConsumerError::Utf8(err)
    }
}
impl From<RecvError> for ConsumerError {
    fn from(err: RecvError) -> Self {
        ConsumerError::Recv(err)
    }
}
impl From<String> for ConsumerError {
    fn from(err: String) -> Self {
        ConsumerError::Other(err)
    }
}
