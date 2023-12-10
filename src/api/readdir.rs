#[derive(serde::Deserialize)]
struct ReaddirRequest {
    path: String,
}

/// List a directory.
///
///
/// # Arguments
///
/// The query body is a json object:
///
/// ```json
/// {
///     "path": "path/to/dir"
/// }
/// ```
///
/// # Returns
///
/// The return json object:
///
/// ```json
/// {
///     "requested_path": "path/to/dir",
///     "entries": [
///         {
///             "name": "file1",
///             "path": "path/to/dir/file1",
///             "type": "FILE",
///             "size": 123,
///             "modified": 123456789,
///         },
///         // more entries list
///     ]
/// }
/// ```
#[actix_web::post("/api/readdir")]
pub async fn post(
    info: actix_web::web::Json<ReaddirRequest>,
) -> actix_web::Result<impl actix_web::Responder> {
    let path = &info.path;
    let ret = listdir(path.as_str()).await?;

    Ok(actix_web::web::Json(ret))
}

async fn listdir(
    path: &str,
) -> Result<frontend::widget::file_explorer::ReaddirResponse, std::io::Error> {
    let mut ret = Vec::<frontend::widget::file_explorer::ReaddirItem>::new();
    let mut entries = tokio::fs::read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let metadata = entry.metadata().await?;

        ret.push(frontend::widget::file_explorer::ReaddirItem {
            f_name: entry.file_name().to_str().unwrap().to_string(),
            f_path: entry.path().to_str().unwrap().to_string(),
            f_type: if entry.file_type().await?.is_dir() {
                "DIR".to_string()
            } else {
                "FILE".to_string()
            },
            f_size: metadata.len(),
            f_modified: match metadata.modified() {
                Ok(t) => t
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                Err(_) => 0,
            },
        });
    }

    return Ok(frontend::widget::file_explorer::ReaddirResponse { entries: ret });
}
