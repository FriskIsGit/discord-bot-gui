use egui::Event;
use twilight_model::channel::Message;

const CDN_DISCORD_DOMAIN: &str = "https://cdn.discordapp.com";
const MEDIA_TENOR: &str = "https://media.tenor.com";
const SUPPORTED_MEDIA: [&str; 4] = ["png", "jpg", "jpeg", "gif"];

pub fn strip_parameters(mut link: String) -> String {
    let index = link.find('?');
    if index.is_none() {
        return link;
    }
    link.truncate(index.unwrap());
    return link;
}
pub fn format_message_no_attachments(msg: &Message) -> String{
    if msg.content.is_empty() {
        format!("[{}]", msg.author.name)
    } else {
        format!("[{}] {}", msg.author.name, msg.content)
    }
}
//let text = util::format_message(&msg);
pub fn format_message(msg: &Message) -> String{
    return if !msg.content.is_empty() {
        if msg.attachments.is_empty() {
            return format!("[{}] {}", msg.author.name, msg.content);
        }
        let link = strip_parameters(msg.attachments[0].url.to_owned());
        format!("[{}] {} {}", msg.author.name, msg.content, link)
    } else {
        if msg.attachments.is_empty() {
            return format!("[{}]", msg.author.name)
        }
        let link = strip_parameters(msg.attachments[0].url.to_owned());
        format!("[{}] {}", msg.author.name, link)
    }
}
pub fn is_domain_trusted(link: &String) -> bool {
    link.starts_with(CDN_DISCORD_DOMAIN) || link.starts_with(MEDIA_TENOR)
}
pub fn pasted_image(ctx: &egui::Context) -> bool {
    return ctx.input(|i| {
        for key in &i.events {
            match key {
                Event::Paste(_) => {
                    return false;
                }
                Event::Key {key, pressed, repeat, modifiers} => {
                    return key == &egui::Key::V && *pressed && modifiers.ctrl &&
                        !*repeat && (modifiers.command || modifiers.mac_cmd);
                }
                _ => {}
            }
        }
        return false;
    });
}
pub fn is_supported_media(link: &String) -> bool {
    for format in SUPPORTED_MEDIA {
        if link.ends_with(format) {
            return true;
        }
    }
    return false;
}