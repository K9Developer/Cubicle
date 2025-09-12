use std::sync::{Arc, Mutex};

pub trait WithLock<T> {
    fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    fn with_read<R>(&self, f: impl FnOnce(&T) -> R) -> R;
}

impl<T> WithLock<T> for Mutex<T> {
    fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut g = self.lock().unwrap();
        f(&mut *g)
    }
    fn with_read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let g = self.lock().unwrap();
        f(&*g)
    }
}

impl<T> WithLock<T> for Arc<Mutex<T>> {
    fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut g = self.lock().unwrap();
        f(&mut *g)
    }

    fn with_read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let g = self.lock().unwrap();
        f(&*g)
    }
}