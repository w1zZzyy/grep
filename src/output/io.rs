pub trait IO {
    fn write(&mut self, path : &str, lines : Vec<String>);
    fn show(&self) -> std::io::Result<()>;
}