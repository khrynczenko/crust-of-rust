use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Sync {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }
    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {}
        // SAFETY: We have a lock so it is not possible for
        // other thread to be changing at the same time
        let new_value = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        new_value
    }
}

fn main() {
    use std::thread;
    let x: &'static _ = Box::leak(Box::new(Mutex::new(0usize)));
    let threads: Vec<_> = (0..100)
        .into_iter()
        .map(move |_| {
            thread::spawn(move || {
                for _ in 0..10000 {
                    x.with_lock(|v| *v += 1);
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }

    assert_eq!(x.with_lock(|v| *v), 100 * 10000);
}
