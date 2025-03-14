use alloc::sync::Arc;
use core::cell::RefCell;

use crate::sync::{Lock, Semaphore};
use crate::thread::{self, current, Thread};

/// Sleep lock. Uses [`Semaphore`] under the hood.
#[derive(Clone)]
pub struct Sleep {
    inner: Semaphore,
    holder: RefCell<Option<Arc<Thread>>>,
}

impl Default for Sleep {
    fn default() -> Self {
        Self {
            inner: Semaphore::new(1),
            holder: Default::default(),
        }
    }
}

impl Lock for Sleep {
    fn acquire(&self) {
        // 只需要改这个函数
        // 1. 把我的父亲设为 holder
        let holder = self.holder.borrow().as_ref().cloned();
        if holder.is_some() {
            current().waitfor(holder.clone().unwrap());
        }
        self.inner.down();
        if holder.is_some() {
            current().donewait();
        }

        // 2. 把我的父亲设为 none
        self.holder.borrow_mut().replace(thread::current());
    }

    fn release(&self) {
        assert!(Arc::ptr_eq(
            self.holder.borrow().as_ref().unwrap(),
            &thread::current()
        ));

        self.holder.borrow_mut().take().unwrap();
        self.inner.up();
    }
}

unsafe impl Sync for Sleep {}
