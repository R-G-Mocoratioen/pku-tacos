use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::thread::{Schedule, Thread};

/// FIFO scheduler.

pub struct Fcfs([VecDeque<Arc<Thread>>; 64]);

impl Default for Fcfs {
    fn default() -> Self {
        Self([(); 64].map(|_| VecDeque::new()))
    }
}

impl Schedule for Fcfs {
    fn register(&mut self, thread: Arc<Thread>) {
        // kprintln!(
        //     "adding thread {} into the queue with priority {} and id {}",
        //     thread.name(),
        //     thread.priority(),
        //     thread.id()
        // );
        self.0[thread.priority() as usize].push_back(thread)
    }

    fn put_back(&mut self, thread: Arc<Thread>) {
        self.0[thread.priority() as usize].push_front(thread)
    }

    fn schedule(&mut self) -> Option<Arc<Thread>> {
        //kprintln!("calling fcfs::schedule in thread {}", current().name());
        let mut v: Vec<Arc<Thread>> = Vec::new();
        for p in 0..64 {
            for thread in &self.0[p] {
                v.push(thread.clone());
            }
            self.0[p].clear();
        }
        for thread in &v {
            // kprintln!(
            //     "scheduling 1: thread {} has priority {}",
            //     thread.name(),
            //     thread.priority()
            // );
            self.0[thread.priority() as usize].push_back(thread.clone());
        }
        for p in (0..64).rev() {
            if self.0[p].len() > 0 {
                //kprintln!("chosen");
                return self.0[p].pop_front();
            }
            // let back = self.0[p].back();
            // if back.is_some() {
            //     kprintln!(
            //         "scheduling: choosing thread {} to run",
            //         back.unwrap().name()
            //     );
            //     let u = back.unwrap().clone();
            //     self.0[p].pop_back();
            //     self.0[p].push_front(u.clone());
            //     return Some(u.clone());
            //     // return self.0[p].pop_back();
            // }
        }
        //kprintln!("scheduling: no thread to run");
        None
    }
}
