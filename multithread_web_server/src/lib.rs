use std::{
    sync::{Arc, Mutex, mpsc::{channel, Sender, Receiver}},
    thread::{self, JoinHandle}
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

pub struct ThreadPool{
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            handle: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();
                
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected: shutting down.");
                        break;
                    }
                }
            })),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = channel::<Job>();
        let receiver_rc = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, receiver_rc.clone()));
        }
        ThreadPool { 
            workers,
            sender: Some(sender)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            };
        }
    }
}