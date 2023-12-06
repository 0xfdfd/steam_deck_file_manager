#[derive(rust_embed::RustEmbed)]
#[folder = "assets/"]
struct Assets;

struct BackendData<'a> {
    render: handlebars::Handlebars<'a>,
}

impl BackendData<'_> {
    fn new() -> Self {
        let mut handlebars = handlebars::Handlebars::new();

        {
            let file = Assets::get("index.html").unwrap();
            let data = std::str::from_utf8(file.data.as_ref()).unwrap();
            handlebars
                .register_template_string("index.html", data)
                .unwrap();
        }

        BackendData { render: handlebars }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::metadata::LevelFilter::DEBUG)
        .init();

    actix_web::HttpServer::new(|| {
        return actix_web::App::new()
            .app_data(actix_web::web::Data::new(BackendData::new()))
            .service(assets)
            .service(index);
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index(
    data: actix_web::web::Data<BackendData<'_>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let data = data
        .render
        .render(
            "index.html",
            &serde_json::json!({
                "wasm_name": "frontend"
            }),
        )
        .unwrap();
    let rsp = actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(data);

    return Ok(rsp);
}

#[actix_web::get("/{path}")]
async fn assets(
    path: actix_web::web::Path<String>,
) -> actix_web::Result<impl actix_web::Responder> {
    return Ok(handle_embedded_file(path.as_str()));
}

/// Return an embedded file
///
/// # Arguments
/// + `path`: The path to the file
///
/// # Returns
/// + `actix_web::HttpResponse`: The file
fn handle_embedded_file(path: &str) -> actix_web::HttpResponse {
    match Assets::get(path) {
        Some(content) => actix_web::HttpResponse::Ok()
            .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => actix_web::HttpResponse::NotFound().body("404 Not Found"),
    }
}
