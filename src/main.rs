use std::thread;

mod discord;
mod config;
mod app;
mod backend;

fn main() {
    println!("Finished");

    #[cfg(feature = "sdl_backend")]
    backend::sdl_backend::run_app();

    #[cfg(feature = "eframe_backend")]
    backend::eframe_backend::run_app();
}
