use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::thread::{current, Schedule, Status, Thread};

/// FIFO scheduler.

pub struct Fcfs([VecDeque<Arc<Thread>>; 64]);

impl Default for Fcfs {
    fn default() -> Self {
        Self([
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
        ])
    }
}

impl Schedule for Fcfs {
    fn register(&mut self, thread: Arc<Thread>) {
        kprintln!(
            "adding thread {} into the queue with priority {}",
            thread.name(),
            thread.priority()
        );
        self.0[thread.priority() as usize].push_front(thread)
    }

    fn schedule(&mut self) -> Option<Arc<Thread>> {
        //kprintln!("calling fcfs::schedule in thread {}", current().name());
        let mut v: Vec<Arc<Thread>> = Vec::new();
        for p in 0..64 {
            for thread in &self.0[p] {
                if thread.status() != Status::Dying {
                    v.push(thread.clone());
                }
            }
            self.0[p].clear();
        }
        for thread in &v {
            // kprintln!(
            //     "scheduling: thread {} has priority {}",
            //     thread.name(),
            //     thread.priority()
            // );
            self.0[thread.priority() as usize].push_back(thread.clone());
            //self.0[0].push_back(thread.clone());
        }
        for p in (0..64).rev() {
            let back = self.0[p].back();
            if back.is_some() {
                kprintln!(
                    "scheduling: choosing thread {} to run",
                    back.unwrap().name()
                );
                let u = back.unwrap().clone();
                self.0[p].pop_back();
                self.0[p].push_front(u.clone());
                return Some(u.clone());
                // return self.0[p].pop_back();
            }
        }
        kprintln!("scheduling: no thread to run");
        None
    }
}
