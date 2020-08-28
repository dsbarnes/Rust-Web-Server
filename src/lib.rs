use std::thread;
use std::sync::{ mpsc, Mutex, Arc };


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub enum Message {
    NewJob(Job),
    Terminate,
}


impl Worker {
    pub fn new(
    id: usize,
    receiver: Arc<Mutex<mpsc::Receiver<Message>>>
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                },
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Dropping workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down workder {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// # Panics
    /// The 'new' function fill panic if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<T>(&self, f: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
