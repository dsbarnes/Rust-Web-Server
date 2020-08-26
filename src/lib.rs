use std::thread;


pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
    

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// # Panics
    /// The 'new' function fill panic if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // create threads here
        }
        ThreadPool { threads }
    }

    pub fn execute<T>(&self, f: T)
    where
        T: FnOnce() + Send + 'static,
    {
    }
}
