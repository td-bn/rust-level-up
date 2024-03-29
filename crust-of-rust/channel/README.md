# Channels

`Channels` gives us handles for sender(s) and a receiver.
`Sender(s)` can send the data and the `Receiver` can receive the data.

Rust provides `mpsc`(multiple producer, single consumer) channels in the std lib.


## Types of channels

1. Asynchronous
Infinite capacity. Non blocking send. Uses `Sender`.

2. Synchronous
Fixed size buffer. Blocking till buffer space available. Uses `SyncSender`.


Both Senders are clonable such that multiple threads can simultaneoulsy send to a single consumber.

In both of these channels all data created on the `Sender` half will become available on the `Receiver` 
in the same order that it was sent. 
The `Sender` can be cloned but only one `Receiver` is supported.


## Useful primitives used in channel implementations

### Mutex

A mutual exclusion primitive for protecting shared data. 

This mutex will block threads waiting for the lock to become available.

The data can only be accessed through the `RAII` guards returned from `lock` and `try_lock`, 
which guarantees that the data is only ever accessed when the mutex is locked.


### Condvar

A conditional variable which represents the ability to block a thread such that it
consumes no CPU time while waiting for an event to occur.

Condition variables are typically associated with a boolean predicate (a condition) and a mutex. 
The predicate is always verified inside of the mutex before determining that a thread must block.

`.wait` : Blocks the current thread until this condition variable receives a notification.

`.notify_one`: Wakes up one blocked thread on this condvar.


### Arc

A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.

The type `Arc<T>` provides shared ownership of a value of type T, allocated in the heap. 

