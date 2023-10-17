use twilight_http::Client;
use twilight_model::channel::{Channel, ChannelType};
use twilight_model::guild::Member;
use twilight_model::user::CurrentUserGuild;

pub async fn test(token: String) {
    let client = Client::builder().token(token).build();
    let guilds = get_connected_guilds(&client).await;
    println!("Currently in {} servers:", guilds.len());
    for guild in guilds {
        println!("{}[{}]", guild.id, guild.name);
        let channels = get_channels(&client, guild).await;
        for channel in channels {
            println!("{}", channel.name.unwrap());
        }
    }
}

pub async fn create_client(token: String) -> Client{
    Client::builder().token(token).build()
}

pub async fn get_connected_guilds(client: &Client) -> Vec<CurrentUserGuild>{
    let response = client.current_user_guilds().await.unwrap();
    let guilds_body = response.text().await.expect("Failed to retrieve body as text");
    serde_json::from_str(guilds_body.as_str()).expect("Failed to deserialize a CurrentUserGuild list")
}
pub async fn get_channels(client: &Client, guild: CurrentUserGuild) -> Vec<Channel>{
    let response = client.guild_channels(guild.id).await.unwrap();
    let channels_body = response.text().await.expect("Failed to retrieve body as text");
    serde_json::from_str(channels_body.as_str()).expect("Failed to deserialize a Channel list")
}

pub async fn get_members(client: &Client, guild: CurrentUserGuild, limit: u16) -> Vec<Member>{
    let response = client.guild_members(guild.id)
        .limit(limit).expect("Member limit failed to validate")
        .await.unwrap();
    let members_body = response.text().await.expect("Failed to retrieve body as text");
    serde_json::from_str(members_body.as_str()).expect("Failed to deserialize a CurrentUserGuild list")
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

pub async fn foo<T: ToString>(obj: T){

}
