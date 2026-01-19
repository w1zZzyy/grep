use crossbeam::channel::{self};
use std::{path::Path, sync::Arc, thread::JoinHandle};

use crate::lib::{
    buffer::Buffer, consumer::*, input::Arguments, producer::*, results::*,
};

pub struct Engine {
    producer: Producer,
    consumer: Consumer,
    consumers: Option<Vec<Consumer>>,
    result: Arc<Results>,
    display: ShowIn,
    path: String,
}

impl Engine {
    pub fn new() -> Self {
        let (tx, rx) = channel::unbounded::<Buffer>();
        let args = Arguments::new().unwrap();
        let path = args.search_root;
        let pattern = args.text;
        let res = Arc::new(Results::new());
        let prod = Producer::new(&tx);
        let con = Consumer::new(&rx, &pattern, res.clone());
        let cons: Option<Vec<Consumer>> = match args.threads {
            Some(th) => Some(vec![
                Consumer::new(&rx, &pattern, res.clone());
                th as usize
            ]),
            None => None,
        };
        let display = match args.output_path {
            Some(p) => ShowIn::File(ResultInFile::new(&p).unwrap()),
            None => ShowIn::Terminal(ResultInTerminal::new()),
        };
        Engine {
            producer: prod,
            consumer: con,
            consumers: cons,
            result: res,
            display: display,
            path: path,
        }
    }

    pub fn run(mut self) {
        let handles = self.launch_consumers();

        self.producer.run(Path::new(&self.path)).unwrap();
        drop(self.producer);
        self.consumer.run().unwrap();

        if let Some(handles) = handles {
            for handle in handles {
                handle.join().unwrap();
            }
        }

        self.display.show(&self.result); 
    }

    fn launch_consumers(&mut self) -> Option<Vec<JoinHandle<()>>> {
        if let Some(cons) = self.consumers.take() {
            let mut handles = Vec::new();
            for con in cons {
                let handle = std::thread::spawn(move || {
                    con.run().unwrap()
                });
                handles.push(handle);
            }
            Some(handles)
        } else {
            None
        }
    }
}
