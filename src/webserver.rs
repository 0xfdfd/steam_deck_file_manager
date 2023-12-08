#[derive(Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub homedir: Option<String>,
}

pub struct BackendData {
    pub render: handlebars::Handlebars<'static>,
    pub homedir: String,
}

impl BackendData {
    pub fn new(config: Config) -> Self {
        let mut handlebars = handlebars::Handlebars::new();
        {
            let file = crate::assets::get("index.html").unwrap();
            let data = std::str::from_utf8(file.data.as_ref()).unwrap();
            handlebars
                .register_template_string("index.html", data)
                .unwrap();
        }

        let homedir = match config.homedir {
            Some(v) => v,
            None => home::home_dir()
                .expect("unable to get homedir")
                .to_str()
                .unwrap()
                .to_string(),
        };

        return BackendData {
            render: handlebars,
            homedir: homedir,
        };
    }
}

/// Create a new webserver.
///
/// # Arguments
///
/// + `ip`: The ip to bind to.
/// + `port`: The port to bind to.
pub async fn new(config: Config) -> std::io::Result<()> {
    let addr = format!("{}:{}", config.ip, config.port);

    actix_web::HttpServer::new(move || {
        let ext_data = actix_web::web::Data::new(BackendData::new(config.clone()));

        return actix_web::App::new()
            .app_data(ext_data)
            .service(crate::api::assets::get)
            .service(crate::api::homedir::post)
            .service(crate::api::index::get)
            .service(crate::api::readdir::post)
            .service(crate::api::upload::post);
    })
    .bind(addr)?
    .run()
    .await
}
