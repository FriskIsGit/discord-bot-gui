use serde::{Deserialize, Deserializer, Serialize};
use twilight_model::channel::Channel;
use twilight_model::guild::{GuildPreview, Permissions};
use twilight_model::id::Id;
use twilight_model::id::marker::GuildMarker;
use twilight_model::user::CurrentUserGuild;
use twilight_model::util::ImageHash;
use twilight_util::snowflake::Snowflake;

#[derive(Debug, Clone)]
pub struct Server {
    pub id: u64,
    pub name: String,
    pub icon: Option<ImageHash>,
    pub owner: bool,
    pub permissions: Permissions,
    pub features: Vec<String>, //what's this for

    pub text_channels: Vec<Channel>,
    pub voice_channels: Vec<Channel>,

    pub preview: Option<GuildPreview>,
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
            preview: None,
        }
    }
    pub fn prefetch(&self) {
    }

    pub fn id_marker(&self) -> Id<GuildMarker> {
        Id::new(self.id)
    }
}
