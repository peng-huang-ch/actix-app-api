use futures::{stream, Stream, StreamExt};
use std::{io, path::PathBuf};
use tokio::fs::{self, DirEntry};

pub async fn visit(
    path: impl Into<PathBuf>,
) -> impl Stream<Item = io::Result<DirEntry>> + Send + 'static {
    async fn visit_inner(path: PathBuf, to_visit: &mut Vec<PathBuf>) -> io::Result<Vec<DirEntry>> {
        let mut dir = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(child) = dir.next_entry().await? {
            if child.metadata().await?.is_dir() {
                to_visit.push(child.path());
            } else {
                files.push(child)
            }
        }
        Ok(files)
    }

    stream::unfold(vec![path.into()], |mut to_visit| async {
        let path = to_visit.pop()?;
        let file_stream = match visit_inner(path, &mut to_visit).await {
            Ok(files) => stream::iter(files).map(Ok).left_stream(),
            Err(e) => stream::once(async { Err(e) }).right_stream(),
        };

        Some((file_stream, to_visit))
    })
    .flatten()
}

#[cfg(test)]
mod test {

    use super::visit;
    use futures::StreamExt;

    #[tokio::main]
    #[test]
    async fn test_visit() -> Result<(), Box<dyn std::error::Error>> {
        let filepath = "/Users/phcxc/workpalce/actix-api/logs".to_string();
        let paths = visit(filepath).await;
        paths
            .for_each(|entry| async {
                match entry {
                    Ok(entry) => println!("visiting {:?}", entry),
                    Err(e) => eprintln!("encountered an error: {}", e),
                }
            })
            .await;
        Ok(())
    }
}
