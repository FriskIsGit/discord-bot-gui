use twilight_http::Client;
use twilight_model::channel::{Channel, ChannelType, Message};
use twilight_model::guild::{GuildPreview, Member};
use twilight_model::http::attachment::Attachment;
use twilight_model::id::Id;
use twilight_model::user::CurrentUserGuild;
use crate::discord::guild::Server;

pub async fn test(token: String) {
    let client = Client::builder().token(token).build();
    let guilds = get_connected_servers(&client).await;
    println!("Currently in {} servers:", guilds.len());
    for guild in guilds {
        println!("{}[{}]", guild.id, guild.name);
        /*let channels = get_channels(&client, guild).await;
        for channel in channels {
            println!("{}", channel.name.unwrap());
        }*/
    }
}

pub fn create_client(token: String) -> Client{
    Client::builder().token(token).build()
}

pub async fn get_connected_servers(client: &Client) -> Vec<Server> {
    let response = client.current_user_guilds().await.unwrap();
    let guilds_body = response.text().await.expect(RESPONSE_BODY_ERR);
    let guilds: Vec<CurrentUserGuild> = serde_json::from_str(guilds_body.as_str()).expect(INSTANCE_ERR);
    let mut servers = Vec::with_capacity(guilds.len());
    for guild in guilds {
        servers.push(Server::from(guild));
    }
    return servers;
}
pub async fn get_channels(client: &Client, server_id: u64) -> Vec<Channel> {
    let response = client.guild_channels(Id::new(server_id)).await.unwrap();
    let channels_body = response.text().await.expect(RESPONSE_BODY_ERR);
    serde_json::from_str(channels_body.as_str()).expect(INSTANCE_ERR)
}

pub async fn get_members(client: &Client, guild_id: u64, limit: u16) -> Vec<Member> {
    let response = client.guild_members(Id::new(guild_id))
        .limit(limit).expect(LIMIT_ERR)
        .await.unwrap();
    let members_body = response.text().await.expect(RESPONSE_BODY_ERR);
    serde_json::from_str(members_body.as_str()).expect(INSTANCE_ERR)
}

pub async fn get_messages(client: &Client, channel_id: u64, limit: u16) -> Vec<Message>{
    let result_response = client.channel_messages(Id::new(channel_id))
        .limit(limit).expect(LIMIT_ERR)
        .await;
    if result_response.is_err() {
        return Vec::new();
    }
    let messages_body = result_response.unwrap().text().await.expect(RESPONSE_BODY_ERR);
    serde_json::from_str(messages_body.as_str()).expect(INSTANCE_ERR)
}
pub async fn create_channel(client: &Client, guild_id: u64, name: String) -> Option<Channel> {
    let result_response = client.create_guild_channel(Id::new(guild_id), name.as_str())
        .expect(VALIDATION_ERR).await;
    if result_response.is_err() {
        return None;
    }
    let response_body = result_response.unwrap().text().await.expect(RESPONSE_BODY_ERR);
    Some(serde_json::from_str(response_body.as_str()).expect(INSTANCE_ERR))
}
pub async fn delete_channel(client: &Client, channel_id: u64) -> Option<Channel> {
    let result_response = client.delete_channel(Id::new(channel_id)).await;
    if result_response.is_err() {
        return None;
    }
    let response_body = result_response.unwrap().text().await.expect(RESPONSE_BODY_ERR);
    Some(serde_json::from_str(response_body.as_str()).expect(INSTANCE_ERR))
}
pub async fn send_message(client: &Client, channel_id: u64, content: &str, reply_id: Option<u64>) -> Message {
    let mut new_msg = client.create_message(Id::new(channel_id))
        .content(content).expect(VALIDATION_ERR);
    if let Some(id) = reply_id {
        new_msg = new_msg.reply(Id::new(id));
    }
    let response = new_msg.await.unwrap();
    let body = response.text().await.expect(RESPONSE_BODY_ERR);
    serde_json::from_str(body.as_str()).expect(INSTANCE_ERR)
}
pub async fn delete_message(client: &Client, channel_id: u64, message_id: u64) -> bool {
    let response = client.delete_message(Id::new(channel_id), Id::new(message_id))
        .await.unwrap();
    response.status().is_success()
}
pub async fn edit_message(client: &Client, channel_id: u64, message_id: u64, new_content: String) -> bool {
    let response = client.update_message(
        Id::new(channel_id),
        Id::new(message_id)
    ).content(Some(new_content.as_str())).expect(VALIDATION_ERR).await.unwrap();
    response.status().is_success()
}
pub async fn send_file(client: &Client, channel_id: u64, filename: String, bytes: Vec<u8>) -> Message {
    let attachment = &[Attachment::from_bytes(filename, bytes, 1)];
    let response = client.create_message(Id::new(channel_id))
        .attachments(attachment).expect(VALIDATION_ERR)
        .await.unwrap();
    let body = response.text().await.expect(RESPONSE_BODY_ERR);
    serde_json::from_str(body.as_str()).expect(INSTANCE_ERR)
}
pub async fn get_guild_preview(client: &Client, server_id: u64) -> GuildPreview {
    let response = client.guild_preview(Id::new(server_id)).await.unwrap();
    let body = response.text().await.expect(RESPONSE_BODY_ERR);
    serde_json::from_str(body.as_str()).expect(INSTANCE_ERR)
}

//consumes
pub fn split_into_text_and_voice(mixed_channels: Vec<Channel>) -> (Vec<Channel>, Vec<Channel>) {
    let mut text_channels = vec![];
    let mut voice_channels = vec![];
    for any_channel in mixed_channels {
        match any_channel.kind {
            ChannelType::GuildText => {
                text_channels.push(any_channel);
            }
            ChannelType::GuildVoice => {
                voice_channels.push(any_channel);
            }
            _ => { continue;}
        }
    }
    return (text_channels, voice_channels);
}

const LIMIT_ERR: &str = "Limit error";
const VALIDATION_ERR: &str = "Failed to validate";
const RESPONSE_BODY_ERR: &str = "Failed to deserialize";
const INSTANCE_ERR: &str = "Failed to instantiate";

pub async fn foo<T: ToString>(obj: T){

}
