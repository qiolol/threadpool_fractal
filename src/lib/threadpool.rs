// This is the thread pool from the book:
// https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html
//
// # usage
//
//     let pool = ThreadPool::new(4); // use 4 threads
//
//     pool.execute(|| {
//         foo();
//     });
//
// where `foo()` is the function to parallelize
//
// # example
//
// to execute `foo()` `N` times in parallel with 4 threads
// where `foo()` increments a number, `i`:
//
//     fn foo(i: Arc<Mutex<i32>>) {
//         *i.lock().unwrap() += 1;
//     }
//
//     let i = Arc::new(Mutex::new(0));
//     let pool = ThreadPool::new(4);
//
//     for _ in 0..N {
//         let i_inner = Arc::clone(&i);
//
//         pool.execute(move || {
//             foo(i_inner);
//         });
//     }
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

/// Task to execute per thread
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Wrapper for Job, indicating further work or shutdown
enum Message {
    NewJob(Job),
    Terminate,
}

/// Creates a thread and waits for Messages
#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// Takes jobs and gives them to a pool of threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool, with each thread stored inside a Worker.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    // TODO: make argumentless ctor default to an appropriate number of threads
    // depending on the system's cpu
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Take a task as a closure and send it in a Message to the Workers
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

// when the pool is dropped, our threads should all join to make sure they finish their work.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                // take ownership of the Worker's thread and have it finish
                thread.join().unwrap();
            }
            // if it's None, this Worker has already had its thread cleaned up
        }
    }
}
 
