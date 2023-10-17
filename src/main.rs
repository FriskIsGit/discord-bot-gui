use crate::config::Config;
use crate::discord::twilight_client;

mod discord;
mod config;
mod app;
mod backend;

#[tokio::main]
async fn main() {
    let config = Config::read_config("res/config.json");
    twilight_client::test(config.token).await;
    println!("Finished");

    #[cfg(feature = "sdl_backend")]
    backend::sdl_backend::run_app();

    #[cfg(feature = "eframe_backend")]
    backend::eframe_backend::run_app();
}
