#[derive(serde::Deserialize)]
struct UploadRequest {
    /// The path to upload to.
    path: String,
}

#[actix_web::post("/upload")]
pub async fn post(
    mut payload: actix_multipart::Multipart,
    info: actix_web::web::Query<UploadRequest>,
) -> actix_web::Result<impl actix_web::Responder> {
    use futures_util::TryStreamExt;
    use tokio::io::AsyncWriteExt;

    tracing::debug!("start upload");

    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition.get_filename().unwrap();
        let actual_filepath = format!("{}/{}", info.path, filename);

        tracing::info!("uploading {}", actual_filepath);

        // We do not write to actual file, but instead write to a temporary file.
        // Once the upload is complete, we rename the temporary file to the actual file.
        let temp_filepath = format!(
            "{}/incomplete.{}.upload",
            info.path,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );

        // Create the temporary file.
        let mut f = match tokio::fs::File::create(&temp_filepath).await {
            Ok(f) => f,
            Err(e) => {
                tracing::warn!("create {} failed: {}", temp_filepath, e);
                return Ok(actix_web::HttpResponse::Forbidden().body(e.to_string()));
            }
        };
        // Write the field data to the temporary file.
        while let Some(chunk) = field.try_next().await? {
            f.write_all(&chunk).await?;
        }
        f.shutdown().await?;

        // Rename the temporary file to the actual file.
        tokio::fs::rename(temp_filepath, actual_filepath).await?;
    }

    return Ok(actix_web::HttpResponse::Ok().finish());
}
