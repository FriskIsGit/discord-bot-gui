use twilight_http::Client;
use twilight_model::user::CurrentUserGuild;
use crate::config::Config;

pub async fn run_client() {
    let config = Config::read_config("res/config.json");
    let client = Client::builder().token(config.token).build();
    println!("Connected!");
    let guilds = get_connected_guilds(&client).await;
    println!("Currently in {} servers:", guilds.len());
    for guild in guilds {
        println!("{}[{}]", guild.id, guild.name)
    }

}

pub async fn get_connected_guilds(client: &Client) -> Vec<CurrentUserGuild>{
    let guilds = client.current_user_guilds().await.expect("TODO: panic message");
    let text = guilds.text().await.expect("Failed to retrieve body as text");
    serde_json::from_str(text.as_str()).expect("Failed to deserialize a CurrentUserGuild list")
}

pub async fn foo<T: ToString>(obj: T){

}
