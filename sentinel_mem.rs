use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

pub struct SentinelStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> SentinelStack<T> {
    pub fn new() -> Self {
        Self { head: AtomicPtr::new(ptr::null_mut()) }
    }

    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node { data, next: ptr::null_mut() }));
        loop {
            let current_head = self.head.load(Ordering::Relaxed);
            unsafe { (*new_node).next = current_head; }
            if self.head.compare_exchange(current_head, new_node, Ordering::Release, Ordering::Relaxed).is_ok() {
                break;
            }
        }
    }
}
