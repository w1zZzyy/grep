use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Write,
    sync::Mutex,
};

#[derive(Debug)]
pub struct ResultValue {
    row: usize,
    line: String,
}
impl From<(usize, &str)> for ResultValue {
    fn from((row, line): (usize, &str)) -> Self {
        ResultValue {
            row,
            line: line.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Results {
    data: Mutex<HashMap<String, Vec<ResultValue>>>,
}

impl Results {
    pub fn new() -> Self {
        Results {
            data: Mutex::new(HashMap::new()),
        }
    }
    pub fn push(&self, path: &str, lines: Vec<ResultValue>) {
        if !lines.is_empty() {
            let mut lg = self.data.lock().unwrap();
            lg.insert(path.into(), lines);
        }
    }
    pub fn load(&self) -> String {
        self.data
            .lock()
            .unwrap()
            .iter()
            .map(|iter| parse_result(iter.0, iter.1))
            .collect()
    }
}

fn parse_result(path: &str, lines: &Vec<ResultValue>) -> String {
    let header = format!("Path: {}\nMatches: {}", path, lines.len());
    let body = parse_lines(lines);
    format!("{}\n{}\n", header, body)
}

fn parse_lines(lines: &Vec<ResultValue>) -> String {
    if lines.is_empty() {
        return String::from("No matches.\n");
    }

    let max_row = lines.iter().map(|iter| iter.row).max().unwrap();
    let max_text = lines.iter().map(|iter| iter.line.len()).max().unwrap().min(80);

    let mut output = format!(
        "{:>width$} | {:<text_width$}\n",
        "Row",
        "Text",
        width = max_row.to_string().len(),
        text_width = max_text
    );

    output.push_str(&format!(
        "{}-+-{}\n",
        "-".repeat(max_row.to_string().len()),
        "-".repeat(max_text)
    ));

    for iter in lines {
        let line_text = if iter.line.len() > max_text {
            &iter.line[..max_text]
        } else {
            iter.line.trim()
        };

        output.push_str(&format!(
            "{:>width$} | {:<text_width$}\n",
            iter.row,
            line_text,
            width = max_row.to_string().len(),
            text_width = max_text
        ));
    }

    output
}

pub trait ShowResult {
    fn show(&mut self, result: &Results);
}

pub struct ResultInTerminal;
impl ResultInTerminal {
    pub fn new() -> Self {
        ResultInTerminal {}
    }
}
impl ShowResult for ResultInTerminal {
    fn show(&mut self, result: &Results) {
        print!("{}", result.load());
    }
}

pub struct ResultInFile {
    file: File,
}
impl ResultInFile {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        println!("file");
        let file = OpenOptions::new().write(true).open(path)?;
        Ok(ResultInFile { file: (file) })
    }
}
impl ShowResult for ResultInFile {
    fn show(&mut self, result: &Results) {
        let _ = self.file.write(result.load().as_bytes());
    }
}

pub enum ShowIn {
    Terminal(ResultInTerminal),
    File(ResultInFile),
}

impl ShowIn {
    pub fn show(&mut self, res : &Results) {
        match self {
            ShowIn::File(f) => f.show(res),
            ShowIn::Terminal(t) => t.show(res),
        }
    }
}
