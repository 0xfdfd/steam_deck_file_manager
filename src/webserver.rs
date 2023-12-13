#[derive(Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
}

pub struct BackendData {
    pub render: handlebars::Handlebars<'static>,
}

impl BackendData {
    pub fn new() -> Self {
        let mut handlebars = handlebars::Handlebars::new();
        {
            let file = crate::assets::get("index.html").unwrap();
            let data = std::str::from_utf8(file.data.as_ref()).unwrap();
            handlebars
                .register_template_string("index.html", data)
                .unwrap();
        }

        return BackendData { render: handlebars };
    }
}

/// Create a new webserver.
///
/// # Arguments
///
/// + `ip`: The ip to bind to.
/// + `port`: The port to bind to.
pub fn new(config: Config) -> Result<actix_web::dev::Server, std::io::Error> {
    let addr = format!("{}:{}", config.ip, config.port);

    let srv = actix_web::HttpServer::new(move || {
        let ext_data = actix_web::web::Data::new(BackendData::new());

        return actix_web::App::new()
            .app_data(ext_data)
            .service(crate::api::assets::get)
            .service(crate::api::dirs::post)
            .service(crate::api::index::get)
            .service(crate::api::readdir::post)
            .service(crate::api::upload::post);
    })
    .bind(addr);

    let srv = match srv {
        Ok(v) => v.run(),
        Err(e) => return Err(e),
    };

    return Ok(srv);
}
