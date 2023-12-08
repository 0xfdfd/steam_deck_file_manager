#[actix_web::post("/api/homedir")]
pub async fn post(
    data: actix_web::web::Data<crate::webserver::BackendData>,
) -> impl actix_web::Responder {
    let homedir = &data.homedir;
    tracing::info!("homedir: {}", homedir);

    let obj = serde_json::json!({
        "path": homedir,
    });

    return actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .body(obj.to_string());
}
