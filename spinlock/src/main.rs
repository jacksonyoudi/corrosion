use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::{thread, time};


struct LightLock(AtomicBool);

struct LightGuard<'a> {
    lock: &'a LightLock,
}

impl LightLock {
    pub fn new() -> LightLock {
        LightLock(AtomicBool::new(false))
    }

    pub fn try_lock<'a>(&'a self) -> Option<LightGuard<'a>> {
        let was_locked = self.0.swap(true, Ordering::Acquire);
        if was_locked {
            None
        } else {
            Some(LightGuard {
                lock: self
            })
        }
    }
}

impl<'a> Drop for LightGuard<'a> {
    fn drop(&mut self) {
        self.lock.0.store(false, Ordering::Relaxed);
    }
}


fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = Arc::clone(&spinlock);

    let th = thread::spawn(move || {
        // lock
        spinlock_clone.store(1, Ordering::SeqCst);
        //  do something
        let t = time::Duration::from_secs(3);
        thread::sleep(t);
        //  unlock
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::SeqCst) != 0 {}
    if let Err(panic) = th.join() {
        println!("Thread had an error: {:?}", panic);
    }
}
