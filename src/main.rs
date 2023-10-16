mod app;
mod config;
mod backend;
mod twilight_client;


/*#[tokio::main]
async fn main() {
    twilight_client::run_client().await;
}*/

fn main() {
    #[cfg(feature = "sdl_backend")]
    backend::sdl_backend::run_app();

    #[cfg(feature = "eframe_backend")]
    backend::eframe_backend::run_app();
}
