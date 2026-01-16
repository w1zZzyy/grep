pub struct OutputFormat {
    pub num : usize, 
    pub line : String
}

impl OutputFormat {
    pub fn new(num : usize, line : String) -> Self {
        OutputFormat { num, line }
    }
}