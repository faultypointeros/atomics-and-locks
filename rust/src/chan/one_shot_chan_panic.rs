use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct OneShotChanPanic<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool,
}
unsafe impl<T> Sync for OneShotChanPanic<T> where T: Send {}

impl<T> OneShotChanPanic<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }

    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Ordering::Relaxed) {
            panic!("Can only send message once.");
        }
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    // safety: only call this method once
    // and only after is_ready returns true
    pub fn recv(&self) -> T {
        if !self.ready.swap(false, Ordering::Acquire) {
            panic!("there is no message available");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for OneShotChanPanic<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { (*self.message.get()).assume_init_drop() }
        }
    }
}
