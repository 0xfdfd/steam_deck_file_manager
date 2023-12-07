/// Create a new webserver.
///
/// # Arguments
///
/// + `ip`: The ip to bind to.
/// + `port`: The port to bind to.
pub async fn new(ip: &str, port: u16) -> std::io::Result<()> {
    let addr = format!("{}:{}", ip, port);

    actix_web::HttpServer::new(|| {
        let ext_data = actix_web::web::Data::new(crate::api::BackendData::new());

        return actix_web::App::new()
            .app_data(ext_data)
            .service(crate::api::assets::get)
            .service(crate::api::index::get);
    })
    .bind(addr)?
    .run()
    .await
}
