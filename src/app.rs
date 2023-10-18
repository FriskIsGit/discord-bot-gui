use std::fmt::format;
use std::sync::Arc;
use crate::config::Config;

use egui;
use egui::scroll_area::ScrollBarVisibility;
use egui::{Align, Label, Layout, Sense, TextBuffer, Vec2, Visuals};
use twilight_http::Client;
use tokio::runtime;
use twilight_model::channel::{Channel, Message};
use twilight_util::snowflake::Snowflake;
use crate::discord::twilight_client;
use crate::discord::fetch::Fetch;
use crate::discord::guild::Server;

pub struct DiscordApp {
    tokio: runtime::Runtime, 
    input_text: String,
    current_server: String,
    current_channel: String,
    draw_type: DrawMode,

    selected_server_id: Option<u64>,
    selected_channel_id: Option<u64>,

    servers_fetch: Fetch<Vec<Server>>,
    servers: Vec<Server>,

    channels_fetch: Fetch<(Vec<Channel>, Vec<Channel>)>,
    channels: (Vec<Channel>, Vec<Channel>),

    messages_fetch: Fetch<Vec<Message>>,
    messages: Vec<Message>,

    send_message_fetch: Fetch<Message>,

    client: Arc<Client>,
    config: Config,
}

impl DiscordApp {
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

            selected_server_id: None,
            selected_channel_id: None,

            servers_fetch: Fetch::new(),
            servers: Vec::new(),

            channels_fetch: Fetch::new(),
            channels: (Vec::new(), Vec::new()),

            messages_fetch: Fetch::new(),
            messages: Vec::new(),

            send_message_fetch: Fetch::new(),
            client: Arc::new(twilight_client::create_client(config.token.clone())),
            config,
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
        self.left_most_panel(ctx);
        self.left_inner_panel(ctx);
        self.member_panel(ctx); //right most
        self.chat_panel(ctx); //middle

    }

    pub fn events(&mut self) {
        //fetch servers
        if self.servers_fetch.start() {
            let client = self.client.clone();
            let sender = self.servers_fetch.sender();
            self.tokio.spawn( async move {
                let guilds = twilight_client::get_connected_servers(&client).await;
                sender.send(guilds).expect("Receiver deallocated?");
            });
        }

        let received = self.servers_fetch.receive();
        if received.is_some() {
            self.servers = received.unwrap();
        }

        //fetch channels
        if self.selected_server_id.is_some() && self.channels_fetch.start() {
            let server_id = self.selected_server_id.unwrap();
            let client = self.client.clone();
            let sender = self.channels_fetch.sender();
            self.tokio.spawn( async move {
                let channels = twilight_client::get_channels(&client, server_id).await;
                let split_channels = twilight_client::split_into_text_and_voice(channels);
                sender.send(split_channels).expect("Receiver deallocated?");
            });
        }
        let received = self.channels_fetch.receive();
        if received.is_some() {
            self.channels = received.unwrap();
        }

        //fetch messages
        if self.selected_channel_id.is_some() && self.messages_fetch.start() {
            let channel_id = self.selected_channel_id.unwrap();
            let client = self.client.clone();
            let sender = self.messages_fetch.sender();
            self.tokio.spawn( async move {
                let messages = twilight_client::get_messages(&client, channel_id, 50).await;
                sender.send(messages).expect("Receiver deallocated?");
            });
        }
        let received = self.messages_fetch.receive();
        if received.is_some() {
            self.messages = received.unwrap();
            let channel_id = self.selected_channel_id.unwrap();
            for channel in &self.channels.0 {
                if channel.id == channel_id {
                    self.current_channel = channel.name.clone().unwrap();
                }
            }
        }

        if !self.input_text.is_empty() && self.send_message_fetch.start() {
            let channel_id = self.selected_channel_id.unwrap();
            let content = self.input_text.clone();
            let client = self.client.clone();
            let sender = self.send_message_fetch.sender();
            self.tokio.spawn( async move {
                let message = twilight_client::send_message(&client, channel_id, content.as_str()).await;
                sender.send(message).expect("Receiver deallocated?");
            });
        }
        let received = self.send_message_fetch.receive();
        if received.is_some() {
            println!("Delivered message");
            let msg = received.unwrap();
            self.messages.insert(0, msg);
        }
    }
}

impl DiscordApp {
    pub fn left_most_panel(&mut self, ctx:  &egui::Context){
        egui::SidePanel::left("server_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Friends").clicked() {
                    self.draw_type = DrawMode::Friends;
                }
                if ui.button("Servers").clicked() {
                    self.draw_type = DrawMode::Servers;
                    self.servers_fetch.request();
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
                        for server in &self.servers {
                            let response = ui.add(Label::new(&server.name)
                                .sense(Sense::click()));
                            if response.clicked() {
                                self.channels_fetch.request();
                                self.selected_server_id = Some(server.id);
                                self.current_server = server.name.clone();
                            }
                        }
                    }
                }
            });
        });

    }
    pub fn left_inner_panel(&mut self, ctx:  &egui::Context){
        egui::SidePanel::left("channel_panel").show(ctx, |ui| {
            ui.heading(&self.current_server);
            ui.separator();
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .auto_shrink([false, false])
                .show(ui, |ui| { //show_rows
                    for text in &self.channels.0 {
                        let name = &text.name.to_owned().unwrap();
                        let response = ui.add(Label::new(name).sense(Sense::click()));
                        if response.clicked() {
                            self.messages_fetch.request();
                            self.selected_channel_id = Some(text.id.id());
                        }
                    }
                    ui.separator();
                    for voice in &self.channels.1 {
                        let name = &voice.name.to_owned().unwrap();
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
                    .desired_rows(1)//this field should allow shift+enter
                    .hint_text("Message");

                let response = ui.add(input_field);
                let pressed_enter = ui.input(|i|
                    i.key_pressed(egui::Key::Enter)); // TODO: allow shift+enter

                if !self.input_text.is_empty() && self.selected_channel_id.is_some() && response.lost_focus() && pressed_enter {
                    println!("Sending message: {}", self.input_text);
                    self.send_message_fetch.request();
                }

                egui::ScrollArea::vertical()
                    .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                    .stick_to_bottom(true)
                    .auto_shrink([false, false])
                    .show(ui, |ui| { //show_rows
                        if self.messages.is_empty() {
                            //ignore attachments for now
                            return;
                        }
                        for msg in &self.messages {
                            if msg.content.is_empty() {
                                continue;
                            }
                            let text = format!("[{}] {}", msg.author.name, msg.content);
                            let response = ui.add(Label::new(text).sense(Sense::click()));
                            response.context_menu(|ui| {
                                if ui.button("Copy text").clicked() {
                                    //ui.output().copied_text = text.clone();
                                    ui.close_menu(); //TODO: copy to clipboard(and make selectable?)
                                }
                                if ui.button("Reply").clicked() {
                                    ui.close_menu();
                                }
                            });
                            ui.separator();
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

#[derive(PartialEq)]
enum DrawMode{
    Friends, Servers
}
