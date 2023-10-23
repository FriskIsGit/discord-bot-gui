use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::sync::Arc;
use crate::config::Config;

use egui;
use egui::scroll_area::ScrollBarVisibility;
use egui::{Label, Sense, TextBuffer, Vec2, Visuals};
use twilight_model::guild::Member;
use crate::discord::jobs::{DeleteMessage, EditMessage, GetMembers, GetMessages, Job, SendMessage};
use crate::discord::jobs::GetChannels;

use crate::discord::shared_cache::{ArcMutex, Queue, SharedCache};

pub struct DiscordApp {
    shared_cache: Arc<SharedCache>,
    job_queue: ArcMutex<Queue<Job>>,
    config: Config,

    input_text: String,
    current_server: String,
    current_channel: String,
    draw_type: DrawMode,

    selected_server_id: u64,
    selected_channel_id: u64,
    reply_message_id: u64,
    edited_message_id: u64,
    is_editing: bool,
}

impl DiscordApp {
    pub fn new(ctx: &egui::Context, shared_cache: Arc<SharedCache>, job_queue: ArcMutex<Queue<Job>>, config: Config) -> Self {
        let visuals = Visuals::dark();
        ctx.set_visuals(visuals);

        // Implement dynamic scale changing?
        ctx.set_pixels_per_point(1.66);

        Self {
            shared_cache,
            job_queue,
            config,
            input_text: "".into(),
            current_server: "".into(),
            current_channel: "".into(),
            draw_type: DrawMode::Servers,

            selected_server_id: 0,
            selected_channel_id: 0,
            reply_message_id: 0,
            edited_message_id: 0,
            is_editing: false,
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

    pub fn append_job(&self, job: Job) {
        let mut queue_guard = self.job_queue.guard();
        (*queue_guard).push(job);
    }
}

impl DiscordApp {
    pub fn left_most_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("server_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Friends").clicked() {
                    self.draw_type = DrawMode::Friends;
                }
                if ui.button("Servers").clicked() {
                    self.draw_type = DrawMode::Servers;
                    self.append_job(Job::GetServers);
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
                        let servers = self.shared_cache.servers.guard();
                        for server in &*servers {
                            let response = ui.add(Label::new(&server.name)
                                .sense(Sense::click()));
                            if response.clicked() {
                                self.selected_server_id = server.id;
                                self.current_server = server.name.clone();
                                let job = Job::GetChannels(GetChannels::new(server.id));
                                self.append_job(job);
                            }
                        }
                    }
                }
            });
            // Options should be placed in the left bottom corner of this panel
            ui.separator();
            if ui.button("Options").clicked() {
                println!("options clicked");
            }
        });
    }
    pub fn left_inner_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("channel_panel").show(ctx, |ui| {
            ui.heading(&self.current_server);
            ui.separator();
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let channels = self.shared_cache.channels.guard(); //show_rows
                    for text_channel in &*channels.0 {
                        let name = text_channel.name.clone().unwrap();
                        let response = ui.add(Label::new(name.clone()).sense(Sense::click()));
                        if response.clicked() {
                            let channel_id = text_channel.id.get();
                            let job = Job::GetMessages(GetMessages::new(channel_id, 100));
                            self.current_channel = name;
                            self.selected_channel_id = channel_id;
                            self.append_job(job);
                        }
                    }
                    ui.separator();
                    for voice in &*channels.1 {
                        let name = &voice.name.to_owned().unwrap();
                        let response = ui.add(Label::new(name).sense(Sense::click()));
                    }
                });
        });
    }
    pub fn chat_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("message_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let input_field = egui::TextEdit::multiline(&mut self.input_text)
                    .min_size(Vec2::new(30.0, 30.0))
                    .desired_rows(1)//this field should allow shift+enter
                    .hint_text("Message");
                let response = ui.add(input_field);
                let submitted = ui.input(|i| {
                    i.key_pressed(egui::Key::Enter) && !i.modifiers.shift
                });
                if !self.input_text.is_empty() && self.selected_channel_id != 0 && submitted {
                    response.surrender_focus();
                    response.request_focus();
                    if self.is_editing && self.edited_message_id != 0 && !self.input_text.is_empty() {
                        let msg_edit = EditMessage::new(
                            self.selected_channel_id,
                            self.edited_message_id,
                            self.input_text.to_owned(),
                        );
                        self.is_editing = false;
                        self.append_job(Job::EditMessage(msg_edit));
                        self.input_text.truncate(0);
                    } else {
                        let job = Job::SendMessage(SendMessage::new(self.selected_channel_id, self.input_text.clone()));
                        self.append_job(job);
                        self.input_text.truncate(0);
                    }
                }
                if ui.button("Add file").clicked() {
                    let job = Job::SelectFile;
                    self.append_job(job);
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.current_channel);
            ui.separator();

            let scroll = egui::ScrollArea::vertical().auto_shrink([false, false]).stick_to_bottom(true);
            scroll.show(ui, |ui| { //show_rows
                let messages = self.shared_cache.messages.guard();
                if messages.is_empty() {
                    //ignore attachments for now
                    return;
                }
                let mut reply = None;
                let mut edit_id = 0;
                let mut is_editing = false;
                let mut edited_text = "".into();
                for msg in messages.iter().rev() {
                    let text: String;
                    if msg.content.is_empty() {
                        if msg.attachments.is_empty() {
                            continue;
                        }
                        text = format!("[{}] {}", msg.author.name, msg.attachments[0].url.clone());
                    } else {
                        text = format!("[{}] {}", msg.author.name, msg.content);
                    }

                    let response = ui.add(Label::new(&text).sense(Sense::click()));
                    response.context_menu(|ui| {
                        if ui.button("Reply").clicked() {
                            reply = Some(msg.id.get());
                            ui.close_menu();
                        }
                        if ui.button("Copy text").clicked() {
                            ui.output_mut(|o| o.copied_text = text.clone());
                            ui.close_menu(); //TODO: make selectable?
                        }
                        if ui.button("Edit message").clicked() {
                            is_editing = true;
                            edit_id = msg.id.get();
                            edited_text = msg.content.clone();
                            ui.close_menu();
                        }
                        if ui.button("Delete message").clicked() {
                            let job = Job::DeleteMessage(DeleteMessage::new(msg.channel_id.get(), msg.id.get()));
                            self.append_job(job);
                            ui.close_menu();
                        }
                    });
                    ui.separator();
                }
                if let Some(id) = reply {
                    self.reply_message_id = id;
                }
                if is_editing {
                    self.is_editing = true;
                    self.edited_message_id = edit_id;
                    self.input_text = edited_text;
                }
            });
        });
    }
    pub fn member_panel(&self, ctx: &egui::Context) {
        egui::SidePanel::right("member_panel").show(ctx, |ui| {
            if ui.button("Fetch members").clicked() && self.selected_server_id != 0 {
                let job = Job::GetMembers(GetMembers::new(self.selected_server_id, 200));
                self.append_job(job);
            }
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let members = self.shared_cache.members.guard();
                    for member in &*members {
                        let response = ui.add(Label::new(&member.user.name).sense(Sense::click()));
                        response.context_menu(|ui| {
                            if ui.button("Copy ID").clicked() {
                                ui.output_mut(|o| o.copied_text = member.user.id.get().to_string());
                                ui.close_menu()
                            }
                        });
                    }
                    ui.separator();
                });
        });
    }
    fn strip_parameters(mut link: String) -> String {
        let index = link.find('?');
        if index.is_none() {
            return link;
        }
        link.truncate(index.unwrap());
        return link;
    }
}

#[derive(PartialEq)]
enum DrawMode {
    Friends,
    Servers,
}
