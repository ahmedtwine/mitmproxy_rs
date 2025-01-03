mod app;
mod config;
mod proxy;

use anyhow::Result;
use iced::{window, Application, Settings};
use tracing::info;

use crate::app::MitmproxyDesktop;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Mitmproxy Desktop...");

    // Start the GUI application
    let settings = Settings {
        window: window::Settings {
            size: (1024, 768),
            position: window::Position::Centered,
            ..Default::default()
        },
        flags: (),
        ..Default::default()
    };

    MitmproxyDesktop::run(settings)?;
    Ok(())
}
