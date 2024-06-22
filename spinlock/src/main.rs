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


#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}


fn main() {
    // let mut foo = Foo;
    // let loan = foo.mutate_and_share();
    // foo.share();
    // println!("{:?}", loan);
    println!("{:04}", 42)
}


fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}