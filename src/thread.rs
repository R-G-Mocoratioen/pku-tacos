//! Kernel Threads

mod imp;
pub mod manager;
pub mod scheduler;
pub mod switch;

pub use self::imp::*;
pub use self::manager::Manager;
pub(self) use self::scheduler::{Schedule, Scheduler};

use alloc::sync::Arc;

/// Create a new thread
pub fn spawn<F>(name: &'static str, f: F) -> Arc<Thread>
where
    F: FnOnce() + Send + 'static,
{
    Builder::new(f).name(name).spawn()
}

/// Get the current running thread
pub fn current() -> Arc<Thread> {
    Manager::get().current.lock().clone()
}

/// Yield the control to another thread (if there's another one ready to run).
pub fn schedule() {
    //kprintln!("called this schedule!!!!!!!!!!!!!!!!");
    Manager::get().schedule()
}

/// Gracefully shut down the current thread, and schedule another one.
pub fn exit() -> ! {
    {
        let current = Manager::get().current.lock();

        #[cfg(feature = "debug")]
        kprintln!("Exit: {:?}", *current);

        current.set_status(Status::Dying);
    }

    schedule();

    unreachable!("An exited thread shouldn't be scheduled again");
}

/// Mark the current thread as [`Blocked`](Status::Blocked) and
/// yield the control to another thread
pub fn block() {
    let current = current();
    current.set_status(Status::Blocked);

    #[cfg(feature = "debug")]
    kprintln!("[THREAD] Block {:?}", current);

    schedule();
}

/// Wake up a previously blocked thread, mark it as [`Ready`](Status::Ready),
/// and register it into the scheduler.
pub fn wake_up(thread: Arc<Thread>) {
    assert_eq!(thread.status(), Status::Blocked);
    thread.set_status(Status::Ready);

    #[cfg(feature = "debug")]
    kprintln!("[THREAD] Wake up {:?}", thread);

    Manager::get().register(thread);
}

/// (Lab1) Sets the current thread's priority to a given value
pub fn set_priority(_priority: u32) {
    use crate::sbi::interrupt;
    let old = interrupt::set(false);
    let current = current();
    current.set_priority(_priority);
    // kprintln!(
    //     "set priority of {} to {}",
    //     current.name(),
    //     current.priority()
    // );
    schedule();
    interrupt::set(old);
}

/// (Lab1) Returns the current thread's effective priority.
pub fn get_priority() -> u32 {
    current().priority()
}

/// (Lab1) Make the current thread sleep for the given ticks.
pub fn sleep(ticks: i64) {
    let current: Arc<Thread> = current();
    if ticks <= 0 {
        return;
    }
    use crate::sbi::timer::timer_ticks;

    let start = timer_ticks();
    Manager::get().new_sleep(current.clone(), start + ticks);
    block();
}
