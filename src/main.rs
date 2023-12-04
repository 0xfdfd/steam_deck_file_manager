mod asset;
mod ui;
mod webserver;

#[derive(Debug, clap::Parser, Default, Clone)]
pub struct AppConfig {
    #[arg(long, value_name = "DIR", help = "Override default home directory")]
    home_dir: Option<String>,
}

fn main() {
    use clap::Parser;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::metadata::LevelFilter::DEBUG)
        .init();

    // Parse CLI
    let config = AppConfig::parse();

    // Initialize tokio
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Run Web Server
    let handle = runtime.spawn(crate::webserver::new("127.0.0.1", 8080, config).unwrap());

    // Run UI
    // It should probably be that last thing you call in your main function.
    {
        use iced::Application;

        let settings = iced::Settings {
            window: iced::window::Settings {
                size: (1280, 800),
                resizable: false,
                ..iced::window::Settings::default()
            },
            ..Default::default()
        };
        ui::UI::run(settings).unwrap();
    }

    // Stop.
    handle.abort();
}
