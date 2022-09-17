// use tokio::signal::unix::{signal, SignalKind};
#![type_length_limit = "1422483"]

use hello::configuration::get_configuration;
use hello::startup::Application;

// use crate::config::{};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    // println!("{:?}", configuration);
    let application = Application::build(configuration.clone()).await?;
    application.run_until_stopped().await?;

    Ok(())
    // https://stackoverflow.com/questions/71258916/how-to-gracefully-shutdown-a-warp-server
}
