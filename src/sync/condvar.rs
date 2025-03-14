//! # Condition Variable
//!
//! [`Condvar`] are able to block a thread so that it consumes no CPU time
//! while waiting for an event to occur. It is typically associated with a
//! boolean predicate (a condition) and a mutex. The predicate is always verified
//! inside of the mutex before determining that a thread must block.
//!
//! ## Usage
//!
//! Suppose there are two threads A and B, and thread A is waiting for some events
//! in thread B to happen.
//!
//! Here is the common practice of thread A:
//! ```rust
//! let pair = Arc::new(Mutex::new(false), Condvar::new());
//!
//! let (lock, cvar) = &*pair;
//! let condition = lock.lock();
//! while !condition {
//!     cvar.wait(&condition);
//! }
//! ```
//!
//! Here is a good practice of thread B:
//! ```rust
//! let (lock, cvar) = &*pair;
//!
//! // Lock must be held during a call to `Condvar.notify_one()`. Therefore, `guard` has to bind
//! // to a local variable so that it won't be dropped too soon.
//!
//! let guard = lock.lock(); // Bind `guard` to a local variable
//! *guard = true;           // Condition change
//! cvar.notify_one();       // Notify (`guard` will overlive this line)
//! ```
//!
//! Here is a bad practice of thread B:
//! ```rust
//! let (lock, cvar) = &*pair;
//!
//! *lock.lock() = true;     // Lock won't be held after this line.
//! cvar.notify_one();       // Buggy: notify another thread without holding the Lock
//! ```
//!

use crate::sync::{Lock, MutexGuard, Semaphore};
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::cell::RefCell;
use core::cmp::Ordering;
use thread::Thread;

#[derive(Clone)]
struct ArcSemaThread(Arc<Semaphore>, Arc<Thread>);
impl Ord for ArcSemaThread {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.priority().cmp(&other.1.priority())
    }
}
impl PartialOrd for ArcSemaThread {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ArcSemaThread {
    fn eq(&self, other: &Self) -> bool {
        self.1.priority() == other.1.priority()
    }
}

impl Eq for ArcSemaThread {}
pub struct Condvar(RefCell<VecDeque<ArcSemaThread>>);

unsafe impl Sync for Condvar {}
unsafe impl Send for Condvar {}

impl Condvar {
    pub fn new() -> Self {
        Condvar(Default::default())
    }

    pub fn wait<T, L: Lock>(&self, guard: &mut MutexGuard<'_, T, L>) {
        let sema = Arc::new(Semaphore::new(0));
        use thread::current;
        self.0
            .borrow_mut()
            .push_back(ArcSemaThread(sema.clone(), current().clone()));

        guard.release();
        sema.down();
        guard.acquire();
    }

    /// Wake up one thread from the waiting list
    pub fn notify_one(&self) {
        let mut binding = self.0.borrow_mut();
        let slice = binding.make_contiguous();
        slice.sort();

        if let Some(sema) = binding.pop_back() {
            sema.0.up();
        }
    }

    /// Wake up all waiting threads
    pub fn notify_all(&self) {
        // kprintln!("calls notify all");
        // self.0.borrow().iter().for_each(|s| s.up());
        // // TODO: 按照顺序最大的来
        // self.0.borrow_mut().clear();
        let mut binding = self.0.borrow_mut();
        let slice = binding.make_contiguous();
        slice.sort();
        slice.reverse();
        binding.iter().for_each(|s| s.0.up());
        binding.clear();
    }
}
