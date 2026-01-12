#![allow(static_mut_refs)]
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if !LOCKED.swap(true, Ordering::Acquire) {
        unsafe { DATA.push('!') };
        LOCKED.store(false, Ordering::Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
    println!("{}", unsafe { DATA.as_str() });
}
