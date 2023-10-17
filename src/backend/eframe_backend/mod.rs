use crate::app::DiscordApp;
use crate::config::Config;

use eframe::AppCreator;
use egui::Vec2;

impl eframe::App for DiscordApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render(ctx);
        self.events();
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

    let app_creator: AppCreator = Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut window = DiscordApp::new(&cc.egui_ctx, config);
        window.setup();
        Box::new(window)
    });

    // Blocks the main thread.
    let _ = eframe::run_native("Discord App", options, app_creator);
    println!("Goodbye");
} 
