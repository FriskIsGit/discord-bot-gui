use crate::config::Config;

use egui;
use egui::scroll_area::ScrollBarVisibility;
use egui::{Align, Label, Layout, Sense, Vec2, Visuals};
use twilight_http::Client;
use crate::discord::twilight_client;
use tokio::runtime;

pub struct MovieApp {
    tokio: runtime::Runtime, 
    input_text: String,
    current_server: String,
    current_channel: String,
    draw_type: DrawMode,
    config: Config,
}

impl MovieApp {
    pub fn new(ctx: &egui::Context, config: Config) -> Self {
        let visuals = Visuals::dark();
        ctx.set_visuals(visuals);

        // Implement dynamic scale changing?
        ctx.set_pixels_per_point(1.66);

        let tokio_runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        Self {
            tokio: tokio_runtime,
            input_text: "".into(),
            current_server: "Socialites".into(),
            current_channel: "#general".into(),
            draw_type: DrawMode::Servers,
            config,
        }
    }

    pub fn setup(&mut self) {
        // REMOVE ME:
        let token = self.config.token.clone();
        self.tokio.spawn(async move {
            twilight_client::test(token).await;
        });

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
        self.left_most_panel(ctx);
        self.left_inner_panel(ctx);
        self.member_panel(ctx); //right most
        self.chat_panel(ctx); //middle
    }
}

impl MovieApp {
    pub fn left_most_panel(&mut self, ctx:  &egui::Context){
        egui::SidePanel::left("server_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Friends").clicked() {
                    self.draw_type = DrawMode::Friends;
                }
                if ui.button("Servers").clicked() {
                    self.draw_type = DrawMode::Servers;
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                match self.draw_type {
                    DrawMode::Friends => {
                        for i in 0..35 {
                            ui.label(format!("friends{}", i));
                        }
                    }
                    DrawMode::Servers => {
                        for i in 0..15 {
                            ui.label(format!("server{}", i));
                        }
                    }
                }

            });
        });

    }
    pub fn left_inner_panel(&self, ctx:  &egui::Context){
        egui::SidePanel::left("channel_panel").show(ctx, |ui| {
            ui.heading(&self.current_server);
            ui.separator();
            ui.vertical(|ui| {
                for i in 0..10 {
                    let name = format!("text_channel{}", i);
                    let response = ui.add(Label::new(name).sense(Sense::click()));
                }
                for i in 0..5 {
                    let name = format!("voice_channel{}", i);
                    let response = ui.add(Label::new(name).sense(Sense::click()));
                }
            });
        });
    }
    pub fn chat_panel(&mut self, ctx:  &egui::Context){
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.current_channel);
            ui.separator();
            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                let input_field = egui::TextEdit::singleline(&mut self.input_text)
                    .min_size(Vec2::new(10.0, 10.0))
                    .hint_text("Message");

                let response = ui.add(input_field);
                let pressed_enter = ui.input(|i| i.key_pressed(egui::Key::Enter));

                if response.lost_focus() && pressed_enter {
                    println!("Sending message: {}", self.input_text);
                }

                egui::ScrollArea::vertical()
                    .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                    .auto_shrink([false, false])
                    .show(ui, |ui| { //show_rows
                        for i in 0..100 {
                            ui.label(format!("message{}", i));
                        }
                    });
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

enum DrawMode{
    Friends, Servers
}
