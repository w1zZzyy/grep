use crate::output::OutputFormat;

pub trait IO {
    fn write(&mut self, path : &str, lines : Vec<OutputFormat>);
    fn show(&self) -> std::io::Result<()>;
}