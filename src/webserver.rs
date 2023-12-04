/// Create a new web server.
///
/// # Arguments
///
/// + `ip`: The IP address to bind to.
/// + `port`: The port to bind to.
pub fn new(
    ip: &str,
    port: u16,
    config: crate::AppConfig,
) -> Result<actix_web::dev::Server, std::io::Error> {
    let addr = format!("{}:{}", ip, port);

    let srv = actix_web::HttpServer::new(move || {
        return actix_web::App::new()
            .app_data(actix_web::web::Data::new(config.clone()))
            .service(index)
            .service(assets)
            .service(fs)
            .service(upload);
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

lazy_static::lazy_static! {
    static ref RENDER : handlebars::Handlebars<'static> = {
        let mut handlebar = handlebars::Handlebars::new();
        handlebar.register_escape_fn(handlebars::no_escape);

        let file = crate::asset::Asset::get("index.html").unwrap();
        let data = std::str::from_utf8(file.data.as_ref()).unwrap();
        handlebar.register_template_string("index.html", data).unwrap();

        handlebar
    };
}

#[derive(serde::Serialize)]
struct HtmlRender {
    fs_data: String,
}

impl HtmlRender {
    fn render(fs_data: &str) -> String {
        let render = HtmlRender {
            fs_data: fs_data.to_string(),
        };

        return RENDER.render("index.html", &render).unwrap();
    }
}

#[actix_web::get("/")]
async fn index(
    config: actix_web::web::Data<crate::AppConfig>,
) -> actix_web::Result<impl actix_web::Responder> {
    // Get home dir
    let home_dir = match config.home_dir.clone() {
        Some(v) => v,
        None => match home::home_dir() {
            Some(v) => v.to_str().unwrap().to_string(),
            None => {
                return Ok(
                    actix_web::HttpResponse::InternalServerError().body("Failed to get home dir")
                );
            }
        },
    };

    tracing::debug!("home_dir: {}", home_dir);

    // Redirect to /fs/<home_dir>
    let rsp = actix_web::HttpResponse::TemporaryRedirect()
        .append_header((
            actix_web::http::header::LOCATION,
            format!("/fs/{}", urlencoding::encode(home_dir.as_str())),
        ))
        .finish();

    return Ok(rsp);
}

#[actix_web::get("/fs/{path}")]
async fn fs(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<String>,
) -> actix_web::Result<impl actix_web::Responder> {
    tracing::info!("fs: {}", path);

    // Query path information.
    let metadata = match tokio::fs::metadata(path.as_ref()).await {
        Ok(v) => v,
        Err(e) => {
            return Ok(actix_web::HttpResponse::NotFound().body(e.to_string()));
        }
    };

    // If it's a file, serve it directly.
    if metadata.is_file() {
        let ret = actix_files::NamedFile::open_async(path.as_ref()).await?;
        return Ok(ret.into_response(&req));
    }

    let fs_list_ret = match listdir(path.as_str()).await {
        Ok(v) => v,
        Err(e) => {
            return Ok(actix_web::HttpResponse::NotFound().body(e.to_string()));
        }
    };
    let fs_json = serde_json::to_string(&fs_list_ret).unwrap();
    let data = HtmlRender::render(fs_json.as_str());

    let rsp = actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(data);

    return Ok(rsp);
}

/// Query arguments for upload a file
#[derive(serde::Deserialize)]
struct UploadRequest {
    /// The path to upload to.
    path: String,
}

#[actix_web::post("/upload")]
async fn upload(
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
