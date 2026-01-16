use crate::output::OutputFormat;
use std::fs;
use std::io;
use std::io::Write;
use std::collections::HashMap;

pub trait OutputTarget {
    fn load(&self, data : &HashMap<String, Vec<OutputFormat>>) -> io::Result<()>;
}

fn parse_lines(lines : &Vec<OutputFormat>) -> String {
    lines
        .iter()
        .map(|item| format!("\t{}) {}\n", item.num, item.line))
        .collect::<String>()
}

fn parse_file(path: &str, lines: &Vec<OutputFormat>) -> Option<String> {
    if lines.is_empty() {
        None
    } else {
        Some(format!(
            "::::File: {}::::\n\
            - Patterns found: {}\n\
            - Lines:\n{}\
            ****************************\n\n",
            path,
            lines.len(),
            parse_lines(lines)
        ))
    }
}

pub struct FileOutput {
    path : String
}
impl FileOutput {
    pub fn new(_path : &str) -> Self {
        FileOutput { path: (_path.to_string()) }
    }
}

impl OutputTarget for FileOutput {
    fn load(&self, data : &HashMap<String, Vec<OutputFormat>>) -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.path)?;

        for (path, lines) in data { 
            if let Some(parsed) = parse_file(path, lines) {
                file.write_all(parsed.as_bytes())?;
            }
        }
        Ok(())
    }
}

pub struct TerminalOutput {}
impl TerminalOutput {
    pub fn new() -> Self { 
        TerminalOutput {}
    }
}

impl OutputTarget for TerminalOutput {
    fn load(&self, data : &HashMap<String, Vec<OutputFormat>>) -> io::Result<()> {
        for (path, lines) in data {
            if let Some(parsed) = parse_file(path, lines) {
                println!("{}", parsed);
            }
        }
        Ok(())
    }
}