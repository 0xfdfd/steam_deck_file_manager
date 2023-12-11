use tracing::level_filters::LevelFilter;

mod api;
mod assets;
mod webserver;

#[derive(Debug, Default, Clone, clap::Parser)]
struct BackendConfig {
    #[arg(long, default_value = "127.0.0.1", help = "The ip to bind to.")]
    ip: String,

    #[arg(long, default_value = "5000", help = "The port to bind to.")]
    port: u16,

    #[arg(long, help = "The path to the home directory.")]
    homedir: Option<String>,

    #[arg(long, default_value = "false", help = "Don't start the gui.")]
    no_gui: bool,
}

fn main() {
    use clap::Parser;

    // Parse command line arguments.
    let config = BackendConfig::parse();

    // Initialize tracing.
    {
        let filter = tracing_subscriber::EnvFilter::builder()
            .with_default_directive(LevelFilter::DEBUG.into())
            .from_env()
            .unwrap()
            .add_directive("hyper::proto=info".parse().unwrap());

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .compact()
            .init();
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Start the webserver.
    let web_config = webserver::Config {
        ip: config.ip.clone(),
        port: config.port,
        homedir: config.homedir.clone(),
    };
    rt.spawn(crate::webserver::new(web_config).unwrap());

    // Local UI.
    if config.no_gui == true {
        rt.block_on(async {
            tokio::signal::ctrl_c().await.unwrap();
        });
    } else {
        let addr = format!("http://{}:{}", config.ip, config.port);
        frontend::start_ui("Steam Deck File Manager", addr.as_str()).unwrap();
    }
}
