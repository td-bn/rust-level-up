use std::sync::{Arc, Condvar, Mutex};
use std::collections::VecDeque;

// Simple implementation of a channel 
//
// Multiple sender single receiver channel
//
// Senders append data to a VecDeque, 
// The Receiver pops off the VecDeque

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    // Lock the thread, add data to VecDeque, release the lock(drop)
    // Wake up the receiver
    pub fn send(&mut self, t: T) {
        let mut data = self.shared.data.lock().unwrap();
        data.queue.push_back(t);
        drop(data);
        self.shared.available.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    // Update number of receivers after obtaining a lock
    fn clone(&self) -> Self {
        let mut data = self.shared.data.lock().unwrap();
        data.senders += 1;
        drop(data);

        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    // Update number of receivers after obtaining a lock
    fn drop(&mut self) {
        let mut data = self.shared.data.lock().unwrap();
        data.senders -= 1;
        let was_last = data.senders == 0;
        drop(data);

        // If there are no more senders, we want to wake up the 
        // receiver in case it is waiting
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    // Batch receive optimazation
    // We'll copy all the contents of the channel into a buffer
    // whenever we are obtaining a lock to read the contents of
    // the channel
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn receive(&mut self) -> Option<T> {
        // If our buffer has some items,
        // no need to obtain lock
        if let Some(t) = self.buffer.pop_front() {
            return Some(t)
        }
        // If there's no data get data swap it with buffer
        let mut data = self.shared.data.lock().unwrap();
        loop {
            match data.queue.pop_front() {
                Some(t) => {
                    if !data.queue.is_empty() {
                        std::mem::swap(&mut data.queue, &mut self.buffer);
                    }
                    return Some(t);
                },
                None if data.senders == 0 => return None,
                None => {
                    data = self.shared.available.wait(data).unwrap();
                }
            }
        }
    }
}

struct Data<T> {
    queue: VecDeque<T>,
    senders: usize,
}

// Data we want to share between channels
struct Shared<T> {
    data: Mutex<Data<T>>,
    available: Condvar,
}


pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // Create a default shared state
    let shared = Shared { 
        data: Mutex::new( Data {
            queue: VecDeque::default(),
            senders: 1
        }),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default(),
        },
    )
}


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn ping_pong() {
        let (mut sn, mut rc) = channel();
        sn.send(42);

        assert_eq!(rc.receive(), Some(42));
    }

    #[test]
    fn closed_sender() {
        let (sn, mut rc) = channel::<()>();
        drop(sn);
        rc.receive();
    }

    // This is the behaviour right now as the receiver is dropped
    //
    // This is a design decision. We need to see how we need to 
    // handle this case. 
    // We can modify send to return an result and handle the error
    // and such
    #[test]
    fn closed_receiver() {
        let (mut sn, rc) = channel::<u32>();
        drop(rc);
        sn.send(42);
    }
}

