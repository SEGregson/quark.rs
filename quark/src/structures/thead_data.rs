use std::sync::{RwLock, Arc, RwLockReadGuard, RwLockWriteGuard};

pub struct ThreadData<T> {
    value: Arc<RwLock<T>>
}

impl<T> Clone for ThreadData<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
} 

// impl<T> PartialEq for ThreadData<T> where  {
//     fn ne(&self, other: &Self) -> bool {
//         !self.eq(other)
//     }

//     fn eq(&self, other: &Self) -> bool {
//         self.value == other.value
//     }
// }

impl<T> ThreadData<T> {
    pub fn new(val: T) -> ThreadData<T> {
        ThreadData { 
            value: Arc::new(
                RwLock::new(val)
            ),
        }
    }

    pub fn try_read_access(&self) -> Option<RwLockReadGuard<'_, T>> {
        match self.value.try_read() {
            Ok(out) => Some(out),
            Err(_) => None,
        }
    }

    pub fn try_write_access(&self) -> Option<RwLockWriteGuard<'_, T>> {
        match self.value.try_write() {
            Ok(out) => Some(out),
            Err(_) => None,
        }
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