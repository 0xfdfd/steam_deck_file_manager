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

/// List a directory.
#[derive(Debug, serde::Serialize)]
struct ListDirItem {
    /// The name of the file.
    #[serde(rename(serialize = "name"))]
    f_name: String,

    /// The absolute path to the file.
    #[serde(rename(serialize = "path"))]
    f_path: String,

    /// The type of the file, 'FILE' or 'DIR'.
    #[serde(rename(serialize = "type"))]
    f_type: String,

    /// The size of the file in bytes.
    #[serde(rename(serialize = "size"))]
    f_size: u64,

    /// The last modified time of the file in seconds.
    #[serde(rename(serialize = "modified"))]
    f_modified: u64,
}

#[derive(Debug, serde::Serialize)]
struct ListdirResult {
    requested_path: String,
    entries: Vec<ListDirItem>,
}

async fn listdir(path: &str) -> Result<ListdirResult, std::io::Error> {
    let mut ret = Vec::<ListDirItem>::new();
    let mut entries = tokio::fs::read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let metadata = entry.metadata().await?;

        ret.push(ListDirItem {
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

    return Ok(ListdirResult {
        requested_path: path.to_string(),
        entries: ret,
    });
}
