mod discord;
mod config;
mod app;
mod backend;

fn main() {
    // let config = Config::read_config("res/config.json");
    println!("Finished");

    #[cfg(feature = "sdl_backend")]
    backend::sdl_backend::run_app();

    #[cfg(feature = "eframe_backend")]
    backend::eframe_backend::run_app();
}
