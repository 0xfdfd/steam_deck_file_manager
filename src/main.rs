mod api;
mod assets;
mod webserver;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::metadata::LevelFilter::DEBUG)
        .init();

    crate::webserver::new("127.0.0.1", 5000).await
}
