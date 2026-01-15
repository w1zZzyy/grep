use crate::output::{io::IO, OutputTarget};
use std::collections::HashMap;

pub struct NonBlockingOutput<T: OutputTarget> {
    results : HashMap<String, Vec<String>>,
    target : T
}
impl<T: OutputTarget> NonBlockingOutput<T> {
    pub fn new(target : T) -> Self {
        NonBlockingOutput { results: (HashMap::new()), target }
    }
}

impl<T: OutputTarget> IO for NonBlockingOutput<T> {
    fn write(&mut self, path : &str, mut lines : Vec<String>) {
        self.results.entry(path.to_string())
                    .or_insert_with(|| Vec::new())
                    .append(&mut lines);
    }

    fn show(&self) -> std::io::Result<()> {
        self.target.load(&self.results)
    }
}

#[cfg(test)]
mod tests {
    use crate::output::{FileOutput, NonBlockingOutput, io::IO};
    use std::{collections::HashMap, fmt::format, fs, io::Read};

    #[test]
    fn file_load() {
        let g_path : String = "file_load.txt".into();
        let data : HashMap<String, Vec<String>> = [
            ("file1".to_string(), vec!["aboba".into(), "bob".into()]),
            ("file2".to_string(), vec![]),
        ].into_iter().collect();

        let _ = fs::File::create(&g_path);
        let targ = FileOutput::new(&g_path);
        let mut output = NonBlockingOutput::new(targ);

        for (path, lines) in data {
            output.write(&path, lines);
        }

        let output_status = output.show();
        assert!(output_status.is_ok());

        let expected = format!(
            "::::File: file1::::\n\
            - Patterns found: 2\n\
            - Lines:\n\
            \t1) aboba\n\
            \t2) bob\n\
            ****************************\n\n"
        );


        let mut file = fs::File::open(g_path).unwrap();
        let mut actual = String::new();
        file.read_to_string(&mut actual).unwrap();

        assert_eq!(actual.trim(), expected.trim());
    }
}