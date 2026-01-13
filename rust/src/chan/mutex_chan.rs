use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct MutexChan<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> MutexChan<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, value: T) {
        self.queue.lock().unwrap().push_back(value);
        self.item_ready.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut guard = self.queue.lock().unwrap();
        loop {
            if let Some(message) = guard.pop_front() {
                return message;
            } else {
                guard = self.item_ready.wait(guard).unwrap();
            }
        }
    }
}
