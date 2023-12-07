/// Get an embedded assets file
///
/// Note: The file must have prefix `/assets/`
///
/// # Arguments
/// + `path`: The path to the file.
#[actix_web::get("/assets/{path}")]
pub async fn get(
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
    match crate::assets::get(path) {
        Some(content) => actix_web::HttpResponse::Ok()
            .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => actix_web::HttpResponse::NotFound().body("404 Not Found"),
    }
}
