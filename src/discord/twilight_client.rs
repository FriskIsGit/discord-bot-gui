use twilight_http::Client;
use twilight_model::channel::Channel;
use twilight_model::guild::Member;
use twilight_model::user::CurrentUserGuild;

pub async fn test(token: String) {
    let client = Client::builder().token(token).build();
    let guilds = get_connected_guilds(&client).await;
    println!("Currently in {} servers:", guilds.len());
    for guild in guilds {
        println!("{}[{}]", guild.id, guild.name);
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
pub async fn member_count(client: &Client, guild: CurrentUserGuild, limit: u16) -> usize{
    let response = client.guild_members(guild.id)
        .limit(limit).expect("Member limit failed to validate")
        .await.unwrap();
    let members_body = response.text().await.expect("Failed to retrieve body as text");
    serde_json::from_str(members_body.as_str()).expect("Failed to deserialize a CurrentUserGuild list")
}

pub async fn foo<T: ToString>(obj: T){

}
