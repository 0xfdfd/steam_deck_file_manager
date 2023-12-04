/// Create a new web server.
///
/// # Arguments
///
/// + `ip`: The IP address to bind to.
/// + `port`: The port to bind to.
pub fn new(ip: &str, port: u16) -> Result<actix_web::dev::Server, std::io::Error> {
    let addr = format!("{}:{}", ip, port);

    let srv = actix_web::HttpServer::new(|| {
        return actix_web::App::new()
            .service(index)
            .service(assets)
            .service(api_home)
            .service(api_listdir)
            .service(api_upload);
    })
    .bind(addr);

    let srv = match srv {
        Ok(v) => v.run(),
        Err(e) => {
            return Err(e);
        }
    };

    return Ok(srv);
}

#[actix_web::get("/")]
async fn index() -> impl actix_web::Responder {
    return handle_embedded_file("index.html");
}

/// Return the home directory
#[actix_web::get("/api/home")]
async fn api_home() -> actix_web::Result<impl actix_web::Responder> {
    Ok(actix_web::web::Json(home::home_dir()))
}

#[derive(serde::Deserialize)]
struct ListDirRequest {
    path: String,
}

#[actix_web::get("/api/listdir")]
async fn api_listdir(
    info: actix_web::web::Query<ListDirRequest>,
) -> actix_web::Result<impl actix_web::Responder> {
    tracing::info!("api_listdir: {}", info.path);

    let ret = listdir(info.path.as_str()).await?;
    Ok(actix_web::web::Json(ret))
}

/// Query arguments for upload a file
#[derive(serde::Deserialize)]
struct UploadRequest {
    /// The path to upload to.
    path: String,
}

#[actix_web::post("/api/upload")]
async fn api_upload(
    mut payload: actix_multipart::Multipart,
    info: actix_web::web::Query<UploadRequest>,
) -> actix_web::Result<impl actix_web::Responder> {
    use iced::futures::TryStreamExt;
    use tokio::io::AsyncWriteExt;

    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition.get_filename().unwrap();
        let actual_filepath = format!("{}/{}", info.path, filename);

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

/// Return an embedded file
///
/// # Arguments
/// + `path`: The path to the file
///
/// # Returns
/// + `actix_web::HttpResponse`: The file
#[actix_web::get("/assets/{path}")]
async fn assets(path: actix_web::web::Path<String>) -> impl actix_web::Responder {
    tracing::debug!("assets: {}", path.as_str());
    return handle_embedded_file(path.as_str());
}

/// Return an embedded file
///
/// # Arguments
/// + `path`: The path to the file
///
/// # Returns
/// + `actix_web::HttpResponse`: The file
fn handle_embedded_file(path: &str) -> actix_web::HttpResponse {
    match crate::asset::Asset::get(path) {
        Some(content) => actix_web::HttpResponse::Ok()
            .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => actix_web::HttpResponse::NotFound().body("404 Not Found"),
    }
}

/// List a directory.
#[derive(Debug, serde::Serialize)]
struct ListDirItem {
    /// The name of the file.
    f_name: String,

    /// The absolute path to the file.
    f_path: String,

    /// The type of the file.
    f_type: String,

    /// The size of the file in bytes.
    f_size: u64,

    /// The last modified time of the file.
    f_modified: u64,
}

#[derive(serde::Serialize)]
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
