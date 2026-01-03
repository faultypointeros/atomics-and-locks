# Hello Threads

## Table Of Contents
- [Rust Threads](#rust-threads)
- [C Threads](#c-threads)


## Rust Threads
A running rust programs consists of collections of threads with their own stack and local state.
Communication between threads can be done through following ways:
- Rust's message passing system: [channels](https://doc.rust-lang.org/stable/std/sync/mpsc/index.html)
- other synchronization primitives
- shared memory data structures (eg: [Arc](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html))

New threads in rust can be spawned using [std::thread::spawn()](https://doc.rust-lang.org/stable/std/thread/fn.spawn.html) function.

When main thread of a rust program terminates, the entire program shuts down even if there are other running threads.

We can wait for a thread to finish by calling `join()` method on the [JoinHandle](https://doc.rust-lang.org/stable/std/thread/struct.JoinHandle.html)
returned by the spawn function. This method returns a `std::thread::Result<T>`; `Ok(T)` on success or `Err` if the joined thread panics.

The `T` is the return type of the function or closure passed to `spawn` function.

We can also pass a closure to `std::thread::spawn()` function allowing us to capture the values to move to the new thread.

### Moving value to closure
The signature of spawn function here is this
```rust
pub fn spawn<f, t>(f: f) -> joinhandle<t>where
    f: fnonce() -> t + send + 'static,
    t: send + 'static,
```
The function or closure passed to spawn and it's return value must me valid
until the lifetime of the whole program ('static).
This is because the thread and its return value may outlive the calling thread.

### Thread Builder
[std::thread::Builder](https://doc.rust-lang.org/stable/std/thread/struct.Builder.html) is a builder for thread which allows changing the default
settings of thread before creating it. These options include the stack size and the name of the thread.
After setting the option, we can call `spawn()` method on it to spawn the thread similar to `std::thread::spawn` which is actually a wrapper around
the `Builder`: `std::thread::Builder::new().spawn().unwrap()`.
The `Builder`'s spawn method returns a `Result`


### Scoped Threads
Remember when the closure passed to spawn couldn't borrow local values because the closure had to be 'static
Well we can use [thread::scope](doc.rust-lang.org/stable/std/thread/fn.scope.html), which gives a
[Scope](https://doc.rust-lang.org/stable/std/thread/struct.Scope.html) object
which we can use to spawn threads. The scope object guarantees that the thread spawned using the scope object will
be joined before the end of scope. Hence we can pass references to values that outlives this scope

> [!NOTE]
> TODO: figure out all these lifetime business related to scope and spawn
```rust
pub fn scope<'env, F, T>(f: F) -> T
where
    F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T,
{}

pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
where
    F: FnOnce() -> T + Send + 'scope,
    T: Send + 'scope,
{}

pub struct Scope<'scope, 'env: 'scope> {
    data: Arc<ScopeData>,
    /// Invariance over 'scope, to make sure 'scope cannot shrink,
    /// which is necessary for soundness.
    ///
    /// Without invariance, this would compile fine but be unsound:
    ///
    /// compile_fail,E0373
    /// std::thread::scope(|s| {
    ///     s.spawn(|| {
    ///         let a = String::from("abcd");
    ///         s.spawn(|| println!("{a:?}")); // might run after `a` is dropped
    ///     });
    /// });
    /// 
    scope: PhantomData<&'scope mut &'scope ()>,
    env: PhantomData<&'env mut &'env ()>,
}
```


## C Threads
### C11 vs Pthreads
> [!NOTE]
> As I'm just learning about these things, so some (probably most) of the things I say here may be wrong, so take it with a grain of salt.

Before C11, there was no ISO standard for threads and friends. But for unix systems, there was POXIS standard that descibed an execution model and
the API to perform various operations realted to threads. C11 introduced the standard for threads which was a simplified version of pthreads. It didn't
have many functionalities and it was optional so many compiler didn't support it.
I don't know what the situation now is, but I think I'll go with the pthreads.

### Pthreads
A single process can contain multiple threads, all executing the same program. They share the same global memory, but each threads have
their own stack.

### Creating Pthread
Creating new threads is done the the function `pthread_create`
```c
int pthread_create(pthread_t *restrict thread,
                          const pthread_attr_t *restrict attr,
                          typeof(void *(void *)) *start_routine,
                          void *restrict arg);
```
On successs, this function returns `0` and before returning, the function saves the thread ID in buffer pointed by `thread.`
On failure it returns an error number and what is stored in `thread` are undefined.

This thread can be used to refer to the newly created thread in subsequent pthread function calls.

One of those function is

### Joining the thread.
```c
int pthread_join(pthread_t thread, void **retval);
```

This function waits for the thread represented by the `thread` to terminate.

Joining a thread multiple times is undefined behaviour.

### Threads Attributes
The `attr` in `pthread_create` function points to an attribute object which is used to create thread with different options.

We can initialize this attr object - `pthread_attr_t` - using the `pthread_attr_init` function. Then we can use various functions to set
appropriate attributes and finally we can pass this attribute object to `pthread_create` calls to create different threads with those attributes.
The attribute object can be destoryed with `pthread_attr_destroy`.
Calling `pthread_attr_init` on an already initialized attribute object is undefined behaviour as is passing a destroyed object to `pthread_create`.

- `pthread_attr_setdetachstate`: set the detach state of the thread. Joinable or Detached.(For more info on detach state see NOTES on `man 3 pthread_create`)

- `pthread_attr_setguardsize`: set the guard size
To see what other attributes are there and how to set them and other details, do the following steps
- open `pthread.h` and search for `pthread_attr_set*`
- read man page for the function you are curious about

### Return Value




## References
- https://doc.rust-lang.org/stable/std/thread/index.html
- https://marabos.nl/atomics/basics.html
- doc.rust-lang.org/stable/std/thread/fn.scope.html
- https://www.gnu.org/software/libc/manual/html_node/POSIX-Threads.html
- man pages for all the pthread functions as well as `man pthreads`
