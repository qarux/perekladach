use actix_multipart::Multipart;
use futures::TryStreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

const DATA_DIR: &'static str = "./data";

pub fn get_source_image_path(uuid: Uuid) -> String {
    format!("{}/pages/{}/source", DATA_DIR, uuid.simple())
}

pub fn get_translated_image_path(uuid: Uuid) -> String {
    format!("{}/pages/{}/translated", DATA_DIR, uuid.simple())
}

pub async fn save_file(mut payload: Multipart, path: &str) -> std::io::Result<()> {
    while let Some(mut field) = payload.try_next()? {
        let mut file = File::create(path).await?;
        while let Some(chunk) = field.try_next().await? {
            file.write_all(&chunk).await?;
        }
    }

    Ok(())
}
