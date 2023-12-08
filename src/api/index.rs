/// Serve index.html for `/`.
#[actix_web::get("/")]
pub async fn get(
    data: actix_web::web::Data<crate::webserver::BackendData>,
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
