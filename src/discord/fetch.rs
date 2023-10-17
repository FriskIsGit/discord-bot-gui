use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};

pub struct Fetch<T>{
    in_progress: bool,
    requested: bool,
    pub sender: Arc<Sender<T>>,
    pub receiver: Receiver<T>
}

impl<T> Fetch<T> {
    pub fn new() -> Self{
        let (sender, receiver) = mpsc::channel();
        Self{
            in_progress: false,
            requested: false,
            sender: Arc::new(sender),
            receiver
        }
    }
    //use in if statement with new thread
    pub fn start(&mut self) -> bool {
        if self.requested {
            self.requested = false;
            self.in_progress = true;
            return true;
        }
        return false;
    }

    pub fn request(&mut self) {
        self.requested = true;
        self.in_progress = false;
    }

    pub fn cancel(&mut self) {
        self.in_progress = false;
    }

    pub fn is_requested(&self) -> bool {
        self.requested
    }

    pub fn is_in_progress(&self) -> bool {
        self.in_progress
    }

    pub fn receive(&self) -> Option<T>{
        if self.requested || !self.in_progress {
            return None;
        }
        return if let Ok(value) = self.receiver.try_recv() {
            Some(value)
        } else {
            None
        }
    }

    pub fn sender(&self) -> Arc<Sender<T>> {
        return self.sender.clone();
    }
}