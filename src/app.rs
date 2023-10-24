use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::config::Config;

use egui;
use egui::scroll_area::ScrollBarVisibility;
use egui::{ImageSource, Label, Sense, TextBuffer, Vec2, Visuals};
use egui::ImageSource::Uri;
use twilight_model::guild::Member;
use crate::discord::event_thread::Ticker;
use crate::discord::jobs::{DeleteMessage, EditMessage, GetMembers, GetMessages, Job, SendFile, SendMessage};
use crate::discord::jobs::GetChannels;

use crate::discord::shared_cache::{ArcMutex, Queue, SharedCache};
use crate::discord::util;

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
    images_pasted: usize,

    longest_render: Duration,
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
            images_pasted: 0,
            is_editing: false,

            longest_render: Duration::from_nanos(1),
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
        let now = Instant::now();
        self.left_most_panel(ctx);
        self.left_inner_panel(ctx);
        self.member_panel(ctx); //right most
        self.chat_panel(ctx); //middle
        let elapsed = now.elapsed();
        println!("{:?} {:?}", elapsed, self.longest_render);
        if elapsed.gt(&self.longest_render) {
            self.longest_render = elapsed;
        }
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
            if ui.button("Placeholder}").clicked() {

            }
            if util::pasted_image(ctx) {
                self.images_pasted += 1;
            }
            ui.label(format!("pastedimgs: {}", self.images_pasted));
        });
    }
    pub fn left_inner_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("channel_panel").show(ctx, |ui| {
            ui.heading(&self.current_server);
            ui.separator();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| { //show_rows
                    let channels = self.shared_cache.channels.guard();
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
                        let job;
                        let taken_text = std::mem::take(&mut self.input_text); // yoinked
                        if self.reply_message_id == 0 {
                            job = SendMessage::new(self.selected_channel_id, taken_text, None);
                        } else {
                            job = SendMessage::new(self.selected_channel_id, taken_text, Some(self.reply_message_id));
                            self.reply_message_id = 0;
                        }
                        self.append_job(Job::SendMessage(job));
                    }
                }
                if ui.button("+").clicked() {
                    let job = Job::SelectFile;
                    self.append_job(job);
                }
                let mut file_bytes = self.shared_cache.file_bytes.guard();
                let mut file_name = self.shared_cache.file_name.guard();
                if !file_bytes.is_empty() && !file_name.is_empty() && self.selected_channel_id != 0 &&
                    ui.button("Send").clicked() {
                    let target_bytes = std::mem::take(&mut *file_bytes);
                    let target_name = std::mem::take(&mut *file_name);
                    let job = SendFile::new(self.selected_channel_id, target_name, target_bytes);
                    self.append_job(Job::SendFile(job));
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.current_channel);
            ui.separator();

            let scroll = egui::ScrollArea::vertical().auto_shrink([false, false]).stick_to_bottom(true);
            scroll.show(ui, |ui| { //show_rows
                let mut messages = self.shared_cache.messages.guard();
                if messages.is_empty() {
                    return;
                }
                let mut reply = None;
                let mut edit_id = 0;
                let mut is_editing = false;
                let mut edited_text = "".into();
                let mut rendered_images = 0;
                let mut message_ids = self.shared_cache.rendered_msg_ids.guard();
                for msg in messages.iter_mut() {
                    if !msg.attachments.is_empty() {
                        msg.attachments[0].url = util::strip_parameters(msg.attachments[0].url.to_owned());
                    }
                }
                for msg in messages.iter().rev() {
                    let text = util::format_message(&msg);
                    let response = ui.add(Label::new(&text).sense(Sense::click()));
                    if !msg.attachments.is_empty() && message_ids.contains(&msg.id.get())  {
                        let link = &msg.attachments[0].url;
                        if util::is_domain_trusted(link) && util::is_supported_media(link) {
                            rendered_images += 1;

                            ui.add(egui::Image::new(Uri(link.into()))
                                .rounding(5.0));
                        }
                    }
                    response.context_menu(|ui| {
                        if ui.button("Reply").clicked() {
                            reply = Some(msg.id.get());
                            ui.close_menu();
                        }
                        if ui.button("Copy text").clicked() {
                            ui.output_mut(|o| o.copied_text = text.clone());
                            ui.close_menu(); //TODO: make selectable?
                        }
                        if !msg.attachments.is_empty() && !(*message_ids).contains(&msg.id.get()) && ui.button("Load image").clicked() {
                            message_ids.push(msg.id.get());
                            ui.close_menu(); //TODO: make selectable?
                        }
                        if ui.button("Edit message").clicked() {
                            is_editing = true;
                            edit_id = msg.id.get();
                            edited_text = msg.content.clone();
                            ui.close_menu();
                        }
                        let _ = ui.button(format!("Attachments: {}", msg.attachments.len()));
                        if msg.reference.is_some() {
                            println!("msg ref is some");
                            let id = msg.reference.clone().unwrap().message_id.unwrap();
                            let _ = ui.button(format!("Replies: {}", id.to_string()));
                        }
                        if ui.button("Copy message ID").clicked() {
                            ui.output_mut(|o| o.copied_text = msg.id.get().to_string());
                            ui.close_menu(); //TODO: make selectable?
                        }
                        if ui.button("Delete message").clicked() {
                            let job = Job::DeleteMessage(DeleteMessage::new(msg.channel_id.get(), msg.id.get()));
                            self.append_job(job);
                            ui.close_menu();
                        }
                    });
                    ui.separator();
                }
                println!("RENDERED: {}", rendered_images);
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
                            if ui.button("Copy name").clicked() {
                                ui.output_mut(|o| o.copied_text = member.user.name.clone());
                                ui.close_menu()
                            }
                            let _ = ui.button("Open DM");
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
}

#[derive(PartialEq)]
enum DrawMode {
    Friends,
    Servers,
}
