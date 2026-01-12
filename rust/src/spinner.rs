//! Spinner: a classic spin lock that keeps busy waiting until it gets what it wants

use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Spinner<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}
unsafe impl<T> Sync for Spinner<T> where T: Send {}

pub struct SpinnerGuard<'a, T> {
    lock: &'a Spinner<T>,
}
// Spinner T is send only when T is send so this is not necessary
unsafe impl<T> Send for SpinnerGuard<'_, T> where T: Send {}
// but since SpinnerGuard's only feild is &Spinner, it will be sync if Spinner is sync
// which we make it when T is send. but through the Deref impls we are giving shared and exclusive
// access to T and not Spinner through the SpinnerGuard so we need to make sure we only make
// Spinnerguard Sync if T is sync
unsafe impl<T> Sync for SpinnerGuard<'_, T> where T: Sync {}

impl<T> Spinner<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinnerGuard<'_, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        SpinnerGuard { lock: self }
    }
}

impl<T> Drop for SpinnerGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<T> Deref for SpinnerGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for SpinnerGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}
