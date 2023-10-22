pub enum Job {
    GetServers,
    SelectFile,
    GetChannels(GetChannels),
    GetMessages(GetMessages),
    GetUserMessages(GetUserMessages),
    GetMembers(GetMembers),
    SendMessage(SendMessage),
    SendFile(SendFile),
}
impl Job{

}


pub struct GetChannels {
    pub server_id: u64,
}
impl GetChannels {
    pub fn new(server_id: u64) -> Self {
        Self{ server_id }
    }
}

pub struct GetMessages {
    pub channel_id: u64,
    pub limit: u16,
}
impl GetMessages {
    pub fn new(channel_id: u64, limit: u16) -> Self {
        Self{ channel_id, limit }
    }
}

pub struct GetUserMessages {
    pub channel_id: u64,
    pub user_id: u64,
    pub limit: u16,
}
impl GetUserMessages {
    pub fn new(channel_id: u64, user_id: u64, limit: u16) -> Self {
        Self{ channel_id, user_id, limit }
    }
}

pub struct GetMembers {
    pub server_id: u64,
    pub limit: u16,
}
impl GetMembers {
    pub fn new(server_id: u64, limit: u16) -> Self {
        Self{ server_id, limit }
    }
}

pub struct SendMessage {
    pub channel_id: u64,
    pub content: String,
}
impl SendMessage {
    pub fn new(channel_id: u64, content: String) -> Self {
        Self{ channel_id, content }
    }
}

pub struct SendFile {
    pub channel_id: u64,
    pub filename: String,
    pub bytes: Vec<u8>
}
impl SendFile {
    pub fn new(channel_id: u64, filename: String, bytes: Vec<u8>) -> Self {
        Self{ channel_id, filename, bytes }
    }
}