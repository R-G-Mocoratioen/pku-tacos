use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::cell::{Cell, RefCell};
use core::cmp::Ordering;

use crate::sbi;
use crate::thread::{self, Thread};

/// Atomic counting semaphore
///
/// # Examples
/// ```
/// let sema = Semaphore::new(0);
/// sema.down();
/// sema.up();
/// ```
#[derive(Clone)]
struct ArcThread(Arc<Thread>);
impl Ord for ArcThread {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.priority().cmp(&other.0.priority())
    }
}
impl PartialOrd for ArcThread {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ArcThread {
    fn eq(&self, other: &Self) -> bool {
        self.0.priority() == other.0.priority()
    }
}

impl Eq for ArcThread {}
#[derive(Clone)]
pub struct Semaphore {
    value: Cell<usize>,
    waiters: RefCell<VecDeque<ArcThread>>,
}
unsafe impl Sync for Semaphore {}
unsafe impl Send for Semaphore {}
impl Semaphore {
    /// Creates a new semaphore of initial value n.
    pub const fn new(n: usize) -> Self {
        Semaphore {
            value: Cell::new(n),
            waiters: RefCell::new(VecDeque::new()),
        }
    }

    /// P operation
    pub fn down(&self) {
        let old = sbi::interrupt::set(false);

        // Is semaphore available?
        while self.value() == 0 {
            // `push_front` ensures to wake up threads in a fifo manner
            self.waiters
                .borrow_mut()
                .push_front(ArcThread(thread::current()));

            // Block the current thread until it's awakened by an `up` operation
            thread::block();
        }
        self.value.set(self.value() - 1);
        //kprintln!("after down: sema.count = {}", self.value());

        sbi::interrupt::set(old);
    }

    /// V operation
    pub fn up(&self) {
        let old = sbi::interrupt::set(false);
        let count = self.value.replace(self.value() + 1);
        //kprintln!("before up: sema.count = {}", count);

        let mut binding = self.waiters.borrow_mut();
        let slice = binding.make_contiguous();
        slice.sort();

        // Check if we need to wake up a sleeping waiter
        if let Some(thread) = binding.pop_back() {
            assert_eq!(count, 0);

            //kprintln!("up thread {}", thread.0.id());

            thread::wake_up(thread.0.clone());
            use thread::schedule;
            drop(binding);
            schedule();
        }
        sbi::interrupt::set(old);
    }

    /// Get the current value of a semaphore
    pub fn value(&self) -> usize {
        self.value.get()
    }
}
