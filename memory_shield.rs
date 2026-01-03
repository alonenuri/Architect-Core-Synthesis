use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

pub struct NanoShield<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> NanoShield<T> {
    pub fn new() -> Self {
        Self { head: AtomicPtr::new(ptr::null_mut()) }
    }

    pub fn secure_push(&self, data: T) {
        let node = Box::into_raw(Box::new(Node { data, next: ptr::null_mut() }));
        loop {
            let current = self.head.load(Ordering::Acquire);
            unsafe { (*node).next = current; }
            if self.head.compare_exchange(current, node, Ordering::Release, Ordering::Relaxed).is_ok() {
                break;
            }
        }
    }
}
