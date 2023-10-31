pub enum Job {
    GetServers,
    SelectFile,
    GetGuildPreview(GetGuildPreview),
    GetChannels(GetChannels),
    GetMessages(GetMessages),
    GetUserMessages(GetUserMessages),
    GetMembers(GetMembers),
    SendMessage(SendMessage),
    EditMessage(EditMessage),
    DeleteMessage(DeleteMessage),
    SendFile(SendFile),
    CreateChannel(CreateChannel),
    DeleteChannel(DeleteChannel),
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
    pub reply_id: Option<u64>,
}
impl SendMessage {
    pub fn new(channel_id: u64, content: String, reply_id: Option<u64>) -> Self {
        Self{ channel_id, content, reply_id}
    }
}

pub struct EditMessage {
    pub channel_id: u64,
    pub message_id: u64,
    pub new_content: String,
}
impl EditMessage {
    pub fn new(channel_id: u64, message_id: u64, new_content: String) -> Self {
        Self{ channel_id, message_id, new_content }
    }
}

pub struct DeleteMessage {
    pub channel_id: u64,
    pub message_id: u64,
}
impl DeleteMessage {
    pub fn new(channel_id: u64, message_id: u64) -> Self {
        Self{ channel_id, message_id }
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

pub struct CreateChannel {
    pub server_id: u64,
    pub name: String,
}
impl CreateChannel {
    pub fn new(server_id: u64, name: String) -> Self {
        Self{ server_id, name }
    }
}

pub struct DeleteChannel {
    pub channel_id: u64,
}
impl DeleteChannel {
    pub fn new(channel_id: u64) -> Self {
        Self{ channel_id }
    }
}

pub struct GetGuildPreview {
    pub server_id: u64,
}
impl GetGuildPreview {
    pub fn new(server_id: u64) -> Self {
        Self{ server_id }
    }
}