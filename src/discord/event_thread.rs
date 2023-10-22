use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::{runtime, time};
use twilight_http::Client;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::thread;
use native_dialog::FileDialog;
use crate::discord::jobs::{GetChannels, GetMessages, Job, SendMessage};
use crate::discord::shared_cache::{ArcMutex, Queue, SharedCache};
use crate::discord::twilight_client;

pub struct EventController {
    pub idling: bool,
    tokio: runtime::Runtime,
    shared_data: Arc<SharedCache>,
    job_queue: ArcMutex<Queue<Job>>,
    client: Arc<Client>
}

impl EventController{
    pub fn new(shared_data: Arc<SharedCache>, job_queue: ArcMutex<Queue<Job>>, token: String) -> Self {
        let tokio_runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        Self{
            shared_data,
            job_queue,
            client: Arc::new(twilight_client::create_client(token)),
            idling: false,
            tokio: tokio_runtime,
        }
    }
    //should be launched on a separate thread
    pub fn idle(&mut self){
        if self.idling {
            return;
        }
        self.idling = true;
        let mut ticker = Ticker::new(Duration::from_millis(50));
        while self.idling {
            ticker.tick();
            self.receive_events();
            self.do_jobs();
            println!("T:{}", ticker.ticked)
        }
    }

    fn receive_events(&mut self) {
    }
    fn do_jobs(&mut self) {
        if (*self.job_queue.guard()).is_empty() {
           return;
        }
        match self.take_job() {
            Job::GetServers => {
                self.get_servers();
            }
            Job::GetChannels(channel_fetch) => {
                self.get_channels(channel_fetch)
            }
            Job::GetMessages(msg_fetch) => {
                self.get_messages(msg_fetch)
            }
            Job::GetUserMessages(user_msg_fetch) => {

            }
            Job::GetMembers(member_fetch) => {

            }
            Job::SendMessage(msg_send) => {
                self.send_message(msg_send)
            }
            Job::SelectFile => {
                self.select_file()
            }
            Job::SendFile(file_send) => {}
        }
    }
    fn get_servers(&mut self) {
        let client = self.client.clone();
        let cache = self.shared_data.clone();
        self.tokio.spawn( async move {
            let guilds = twilight_client::get_connected_servers(&client).await;
            *cache.servers.guard() = guilds;
        });
    }
    fn get_channels(&self, channel_fetch: GetChannels) {
        let client = self.client.clone();
        let cache = self.shared_data.clone();
        self.tokio.spawn(async move {
            let channels = twilight_client::get_channels(&client, channel_fetch.server_id).await;
            let split_channels = twilight_client::split_into_text_and_voice(channels);
            *cache.channels.guard() = split_channels;
        });
    }
    fn get_messages(&self, msg_fetch: GetMessages) {
        let client = self.client.clone();
        let cache = self.shared_data.clone();
        self.tokio.spawn(async move {
            let messages = twilight_client::get_messages(&client, msg_fetch.channel_id, msg_fetch.limit).await;
            *cache.messages.guard() = messages;
        });
    }
    fn send_message(&self, msg_send: SendMessage) {
        let client = self.client.clone();
        let cache = self.shared_data.clone();
        self.tokio.spawn(async move {
            let message = twilight_client::send_message(&client, msg_send.channel_id, msg_send.content.as_str()).await;
            *cache.msg_sent.guard() = Some(message.clone());
            (*cache.messages.guard()).insert(0, message);
        });
    }
    fn select_file(&self) {
        let cache = self.shared_data.clone();
        self.tokio.spawn(async move {
            let path = FileDialog::new()
                .set_location("~/Desktop")
                .show_open_single_file()
                .unwrap();

            let path = match path {
                Some(path) => path,
                None => return,
            };
            let file = File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).expect("Couldn't read all bytes?");
            *cache.file_bytes.guard() = bytes;
        });
    }

    pub fn take_job(&self) -> Job {
        let mut queue_guard = self.job_queue.guard();
        (*queue_guard).take()
    }
}

//non-async ticker/scheduler
struct Ticker {
    pub period: Duration,
    last_tick: Instant,
    ticked: usize,
}
impl Ticker {
    // join ticks at some point?
    pub fn new(period: Duration) -> Self {
        Self{ period, last_tick: Instant::now(), ticked: 0 }
    }

    // sleeps until next tick or returns instantly
    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = self.last_tick.elapsed();
        if delta >= self.period {
            self.last_tick = now;
            self.ticked += 1;
            return;
        }
        thread::sleep(self.period - delta);
        self.last_tick = Instant::now();
        self.ticked += 1;
    }
}