use std::sync::{Arc, Mutex, MutexGuard};
use twilight_model::channel::{Channel, Message};
use twilight_model::guild::Member;
use crate::discord::guild::Server;

pub struct SharedCache {
    pub servers: ArcMutex<Vec<Server>>,
    pub channels: ArcMutex<(Vec<Channel>, Vec<Channel>)>, // should be cached per server

    pub messages: ArcMutex<Vec<Message>>, // should be cached per channel
    pub members: ArcMutex<Vec<Member>>, // should be cached per server

    pub file_bytes: ArcMutex<Vec<u8>>,
    pub file_name: ArcMutex<String>,

    pub rendered_msg_ids: ArcMutex<Vec<u64>>, // cache for UI
    // config: Config,
    // Temporary?
}
impl SharedCache {
    pub fn new() -> Self{
        Self{
            servers: ArcMutex::new(vec![]),
            channels: ArcMutex::new((vec![], vec![])),
            messages: ArcMutex::new(vec![]),
            members: ArcMutex::new(vec![]),
            file_bytes: ArcMutex::new(vec![]),
            rendered_msg_ids: ArcMutex::new(vec![]),
            file_name: ArcMutex::new("".into()),
        }
    }
}

#[derive(Debug)]
pub struct ArcMutex<T> {
    data: Arc<Mutex<T>>
}
impl<T> ArcMutex<T> {
    pub fn new(value: T) -> Self{
        Self{
            data: Arc::new(Mutex::new(value))
        }
    }
    // Always returns the guard regardless of poisoning
    pub fn guard(&self) -> MutexGuard<T> {
        return match self.data.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
    }
}
impl<T> Clone for ArcMutex<T> {
    // deriving Clone can result in unsatisfied trait bounds
    fn clone(&self) -> Self {
        Self{
            data: self.data.clone()
        }
    }
}

#[derive(Debug)]
pub struct Queue<T>{
    pub vec: Vec<T>,
}
impl<T> Queue<T>{
    pub fn new() -> Self {
        Self{ vec: vec![] }
    }

    pub fn take(&mut self) -> T {
        self.vec.remove(0)
    }

    pub fn push(&mut self, element: T){
        self.vec.push(element);
    }

    pub fn clear(&mut self){
        self.vec.truncate(0);
    }
    pub fn size(&mut self) -> usize {
        self.vec.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}