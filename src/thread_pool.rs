use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::error::Error;

#[derive(Debug)]
pub struct PoolError(PoolErrorType);

#[derive(Debug)]
pub enum PoolErrorType {
    SizeIsZero,
}

impl std::fmt::Display for PoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            PoolErrorType::SizeIsZero => write!(f,"The size of the pool cannot be zero"),
            _ => write!(f,"How did we get here?")
        }
    }
}

impl Error for PoolError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

pub struct ThreadPool {
    sender: Sender<Message>,
    workers: Vec<Worker>
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of thread in the pool.
    pub fn new(size: usize) -> Result<ThreadPool, PoolError> {
        if size == 0 {
            return Err(PoolError(PoolErrorType::SizeIsZero));
        }

        let  (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
    
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
    
        Ok(Self {
            sender,
            workers,
        })
    }

    pub fn execute<F>(&self, f: F) 
        where 
            F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}

impl Worker {
    /// Creates a new Worker.
    ///
    /// Holds an id and is in charge of the thread.
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let recv_lock = receiver.lock().unwrap();
            let message = recv_lock.recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Terminating worker {}!", id);
                    break;
                }
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}
