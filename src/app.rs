use crate::config::Config;

use egui;
use egui::scroll_area::ScrollBarVisibility;
use egui::Visuals;

pub struct MovieApp {
    input_text: String,
    current_server: String,
    current_channel: String,
}

impl MovieApp {
    pub fn new(ctx: &egui::Context, config: Config) -> Self {
        let visuals = Visuals::dark();
        ctx.set_visuals(visuals);

        // Implement dynamic scale changing?
        ctx.set_pixels_per_point(1.66);

        Self {
            input_text: "".into(),
            current_server: "Socialites".into(),
            current_channel: "general".into()
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
        self.server_panel(ctx); //left most
        self.channels_panel(ctx); //left inner
        self.chat_panel(ctx); //middle
        self.member_panel(ctx); //right most
    }
}

impl MovieApp {
    pub fn server_panel(&self, ctx:  &egui::Context){
        egui::SidePanel::left("server_panel").show(ctx, |ui| {
            ui.vertical(|ui| {

            });
        });

    }
    pub fn channels_panel(&self, ctx:  &egui::Context){
        egui::SidePanel::left("channel_panel").show(ctx, |ui| {
            ui.heading(&self.current_server);
            ui.separator();
            ui.vertical(|ui| {

            });
        });
    }
    pub fn chat_panel(&self, ctx:  &egui::Context){
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.current_channel);
            ui.separator();
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .auto_shrink([false, false])
                .show(ui, |ui| { //show_rows
                for i in 0..100 {
                    ui.label(format!("messsage{}", i));
                }
            });
        });
    }
    pub fn member_panel(&self, ctx:  &egui::Context){
        egui::SidePanel::right("member_panel").show(ctx, |ui| {
            ui.vertical(|ui| {

            });
        });
    }
}


