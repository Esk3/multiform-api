
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct BestillingsId<T: Clone> {
    pub inner: T,
    pub ids: Arc<Mutex<i32>>,
}

impl<T: Clone> BestillingsId<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            ids: Arc::new(Mutex::new(0)),
        }
    }
    fn next_id(&self) -> i32 {
        let mut lock = self.ids.lock().unwrap();
        *lock += 1;
        *lock
    }
}
