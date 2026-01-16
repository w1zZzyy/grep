use crate::output::{io::IO, OutputTarget, OutputFormat};
use std::collections::HashMap;

pub struct NonBlockingOutput<T: OutputTarget> {
    results : HashMap<String, Vec<OutputFormat>>,
    target : T
}
impl<T: OutputTarget> NonBlockingOutput<T> {
    pub fn new(target : T) -> Self {
        NonBlockingOutput { results: (HashMap::new()), target }
    }
}

impl<T: OutputTarget> IO for NonBlockingOutput<T> {
    fn write(&mut self, path : &str, mut lines : Vec<OutputFormat>) {
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
    use crate::output::{FileOutput, NonBlockingOutput, OutputFormat, io::IO};
    use std::{collections::HashMap, fmt::format, fs, io::Read, process::Output};

    #[test]
    fn file_load() {
        let g_path : String = "file_load.txt".into();
        let data : HashMap<String, Vec<OutputFormat>> = [
            ("file1".to_string(), vec![
                OutputFormat::new(2, "aboba".into()), 
                OutputFormat::new(32, "bob".into())
            ]),
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
            \t2) aboba\n\
            \t32) bob\n\
            ****************************\n\n"
        );


        let mut file = fs::File::open(g_path).unwrap();
        let mut actual = String::new();
        file.read_to_string(&mut actual).unwrap();

        assert_eq!(actual.trim(), expected.trim());
    }
}