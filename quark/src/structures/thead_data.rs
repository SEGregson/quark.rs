use std::sync::{RwLock, Arc, RwLockReadGuard, RwLockWriteGuard, TryLockError};

pub struct ThreadData<T> {
    value: Arc<RwLock<T>>
}

impl<T> Clone for ThreadData<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
} 


impl<T> ThreadData<T> {
    pub fn new(val: T) -> ThreadData<T> {
        ThreadData { 
            value: Arc::new(
                RwLock::new(val)
            ),
        }
    }

    pub fn try_read_access(&self) -> Result<RwLockReadGuard<'_, T>, TryLockError<RwLockReadGuard<'_, T>>>  {
        self.value.try_read() 
    }

    pub fn try_write_access(&self) -> Result<RwLockWriteGuard<'_, T>, TryLockError<RwLockWriteGuard<'_, T>>> {
        self.value.try_write() 
    }

    pub fn force_read_access(&self) -> RwLockReadGuard<'_, T> {
        match self.value.try_read() {
            Ok(out) => out,
            Err(_) => panic!(),
        }
    }

    pub fn force_write_access(&self) -> RwLockWriteGuard<'_, T> {
        match self.value.try_write() {
            Ok(out) => out,
            Err(_) => panic!(),
        }
    }
}