use twilight_model::channel::Channel;
use twilight_model::guild::{Member, Permissions};
use twilight_model::user::CurrentUserGuild;
use twilight_model::util::ImageHash;
use twilight_util::snowflake::Snowflake;

pub struct Server {
    pub id: u64,
    pub name: String,
    pub icon: Option<ImageHash>,
    pub owner: bool,
    pub permissions: Permissions,
    pub features: Vec<String>, //what's this for

    pub text_channels: Vec<Channel>,
    pub voice_channels: Vec<Channel>,
}

impl Server{
    pub fn from(guild: CurrentUserGuild) -> Self{
        Self{
            id: guild.id.id(),
            name: guild.name,
            icon: guild.icon,
            owner: guild.owner,
            permissions: guild.permissions,
            features: guild.features,
            text_channels: vec![],
            voice_channels: vec![],
        }
    }

    pub fn prefetch(&self){

    }
}