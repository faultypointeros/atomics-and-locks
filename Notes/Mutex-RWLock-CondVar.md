# Mutexes, Read-Write Locks and Conditional Variables

## Table Of Contents
- [Rust Mutex](#rust-mutex)
  - [Mutex Poisoning](#mutex-poisoning)
- [References](#references)

## Rust Mutex
`std::sync::Mutex<T>` is a mutual exclusion privitive used to protect shared data.
Threads can get exclusive access to the underlying data by locking a mutex. If a thread tries to lock an already locked
mutex, the thread will block waiting for the it to become available.

The `Mutex` has following methods allowing us to work with it:

- creating new `Mutex`: `pub const fn new(t: T) -> Mutex<T>`
- acquire a mutex or try blocking: `pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>`
- acquire a muted (non blocking): `pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>>`
- check if a mutex is poisoned: `is_poisoned()`
- clear the poisoned state: `clear_poison()`
- consume mutex and return the underlying data: `into_inner()`
- get mutable reference to underlying data: `get_mut()`

### Mutex Poisoning
If the thread holding a mutex panics, the mutex becomes poisoned. Other threads can lock a poisoned mutex, but instead of getting a MutexGuard
they get back `PoisonError`. This is why `lock` and `try_lock` return a `Result`.

- https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#poisoning
- https://sunshowers.io/posts/on-poisoning/

## Posix Mutex
> A mutex is a MUTual EXclusion device, and is useful for protecting shared data structures from concurrent modifications, and
> and implementing critical sections and monitors
> ~ man page for pthread_mutex_init

## References
- https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html
