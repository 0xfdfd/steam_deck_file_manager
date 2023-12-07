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
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    use clap::Parser;

    // Initialize tracing.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::metadata::LevelFilter::DEBUG)
        .init();

    // Parse command line arguments.
    let config = BackendConfig::parse();

    // Start the webserver.
    crate::webserver::new(webserver::Config {
        ip: config.ip,
        port: config.port,
        homedir: config.homedir,
    })
    .await
}
