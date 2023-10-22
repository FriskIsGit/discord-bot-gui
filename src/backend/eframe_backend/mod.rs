use std::sync::Arc;
use std::thread;
use crate::app::DiscordApp;
use crate::discord::event_thread::EventController;
use crate::discord::shared_cache::{ArcMutex, Queue, SharedCache};
use crate::config::Config;

use eframe::AppCreator;
use egui::Vec2;

impl eframe::App for DiscordApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render(ctx);
    }
}

pub fn run_app() {
    println!("Running!");
    let config = Config::read_config("res/config.json");

    let options = eframe::NativeOptions {
        min_window_size: Some(Vec2::new(30.0, 30.0)),
        drag_and_drop_support: true,
        ..Default::default()
    };

    let shared_cache = Arc::new(SharedCache::new()); // shared in two places
    let job_queue = ArcMutex::new(Queue::new()); // shared in two places
    let mut event_controller = EventController::new(
        shared_cache.clone(),
        job_queue.clone(),
        config.token.clone());
    thread::spawn(move || {
        event_controller.idle();
    });
    let app_creator: AppCreator = Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut window = DiscordApp::new(&cc.egui_ctx, shared_cache, job_queue, config);
        window.setup();
        Box::new(window)
    });

    // Blocks the main thread.
    let _ = eframe::run_native("Discord App", options, app_creator);
    println!("Goodbye");
} 
