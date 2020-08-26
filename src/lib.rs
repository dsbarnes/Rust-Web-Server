pub struct ThreadPool;
    

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        ThreadPool
    }

    pub fn execute<T>(&self, f: T)
    where
        T: FnOnce() + Send + 'static,
    {
    }
}
