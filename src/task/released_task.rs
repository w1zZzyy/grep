use std::usize;

use crate::output::{IO, OutputFormat};
use crate::task::execute::*;
use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};

pub struct ReleasedTask<'a, 'b, T: IO> {
    text : String, 
    pattern : &'a str,
    output : Option<&'b mut T>, 
    path : String
}

impl<'a, 'b, T: IO> ReleasedTask<'a, 'b, T> {
    pub fn from_text(text : String, pattern : &'a str, path : &str) -> Self {
        ReleasedTask { text, pattern, output : None, path : path.to_string() }
    }
    pub async fn from_path(path : &str, pattern : &'a str) -> Result<Self, std::io::Error> {
        let mut file = open_file(path).await?;
        let mut text = String::new();
        file.read_to_string(&mut text).await?;
        Ok(ReleasedTask { text, pattern, output : None, path : path.to_string() })
    }
    pub fn set_output(&mut self, output : &'b mut T) {
        self.output = Some(output);
    }
}

#[async_trait(?Send)]
impl<'a, 'b, T: IO> Executable for ReleasedTask<'a, 'b, T> {
    async fn execute(&mut self) -> std::io::Result<()> {
        if let Some(output) = self.output.as_deref_mut() {
            let lines : Vec<OutputFormat> = 
                self.text
                    .lines() 
                    .enumerate()
                    .filter(|s| s.1.contains(self.pattern))
                    .map(|s| OutputFormat::new(s.0 + 1, s.1.to_string()))
                    .collect();

            output.write(&self.path, lines);
            Ok(())
        } else {
            panic!("user forgot to settle output")
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::{fs::File, io::AsyncReadExt};

    use crate::{output::{FileOutput, IO, NonBlockingOutput}, task::{execute::Executable, released_task::ReleasedTask}};

    #[tokio::test]
    async fn released_task_single_thread_test() {
        let output_path = "output_released_single_thread.txt";
        let _ = File::create(&output_path).await; 
        
        let mut output = NonBlockingOutput::new(
            FileOutput::new(&output_path)
        );

        let mut task = ReleasedTask::<NonBlockingOutput<FileOutput>>::from_text(
            "sdafas\ndadada\nda\nsadf\n".into(), 
            "da", 
            "path1"
        );

        task.set_output(&mut output);
        assert!(task.execute().await.is_ok());

        assert!(output.show().is_ok());

        let expected = format!(
            "::::File: path1::::\n\
            - Patterns found: 3\n\
            - Lines:\n\
            \t1) sdafas\n\
            \t2) dadada\n\
            \t3) da\n\
            ****************************\n\n"
        );

        let mut actual = String::new();
        let mut file = tokio::fs::OpenOptions::new().read(true).open(&output_path).await.unwrap();
        let _ = file.read_to_string(&mut actual).await;

        assert_eq!(expected.trim(), actual.trim());
    }
}