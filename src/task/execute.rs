use async_trait::async_trait;
use tokio::fs as tfs;
use tokio::io as tio;

#[async_trait(?Send)]
pub trait Executable {
    async fn execute(&mut self) -> std::io::Result<()>;
}

pub async fn open_file(path : &str) -> Result<tfs::File, std::io::Error> {
    let file = tfs::OpenOptions::new()
            .read(true)
            .open(&path).await?;
    Ok(file)
}