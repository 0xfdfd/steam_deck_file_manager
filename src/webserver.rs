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
            .service(api_listdir);
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

#[actix_web::get("/")]
async fn index() -> impl actix_web::Responder {
    return handle_embedded_file("index.html");
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

#[derive(serde::Serialize)]
struct ListDirItem {
    f_name: String,
    f_path: String,
    f_type: String,
    f_size: u64,
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
        ret.push(ListDirItem {
            f_name: entry.file_name().to_str().unwrap().to_string(),
            f_path: entry.path().to_str().unwrap().to_string(),
            f_type: if entry.file_type().await?.is_dir() {
                "DIR".to_string()
            } else {
                "FILE".to_string()
            },
            f_size: entry.metadata().await.unwrap().len(),
        });
    }

    return Ok(ListdirResult {
        requested_path: path.to_string(),
        entries: ret,
    });
}
