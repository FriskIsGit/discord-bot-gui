use crate::config::Config;

use egui;
use egui::Visuals;

pub struct MovieApp {
    input_text: String,
}

impl MovieApp {
    pub fn new(ctx: &egui::Context, config: Config) -> Self {
        let visuals = Visuals::dark();
        ctx.set_visuals(visuals);

        Self {
            input_text: "".into()
        }
    }

    pub fn setup(&mut self) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        // fonts.font_data.insert(
        //     "my_font".to_owned(),
        //     egui::FontData::from_static(include_bytes!("/fonts/Hack-Regular.ttf")),
        // );

        // Put my font first (highest priority) for proportional text:
        fonts.families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts.families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        // Tell egui to use these fonts:
        // ctx.set_fonts(fonts);
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        // Implement dynamic scale changing?
        ctx.set_pixels_per_point(1.66);

        self.server_panel(ctx); //left most
        self.channels_panel(ctx); //left inner
        self.chat_panel(ctx); //middle
        self.member_panel(ctx); //right most
    }
}

impl MovieApp {
    pub fn server_panel(&self, ctx:  &egui::Context){

    }
    pub fn channels_panel(&self, ctx:  &egui::Context){

    }
    pub fn chat_panel(&self, ctx:  &egui::Context){

    }
    pub fn member_panel(&self, ctx:  &egui::Context){

    }
}


