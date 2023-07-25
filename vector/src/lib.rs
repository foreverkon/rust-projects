#![allow(unused)]
#![allow(non_camel_case_types)]

use std::alloc::{alloc, realloc, Layout};
use std::{
    ops::{Deref, DerefMut},
    ptr::{read, write, NonNull},
};

struct vector<T> {
    data: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> vector<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }

    pub fn into_raw(self) -> *mut T {
        self.data.as_ptr()
    }

    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Self::new_buf(cap),
            len: 0,
            cap,
        }
    }

    pub fn push(&mut self, e: T) {
        self.grow();

        unsafe {
            let ptr = self.data.as_ptr().offset(self.len as isize);
            write(ptr, e);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.empty() {
            None
        } else {
            self.len -= 1;
            unsafe {
                let ptr = self.data.as_ptr().offset(self.len as isize);
                Some(read(ptr))
            }
        }
    }

    fn grow(&mut self) {
        if self.len != self.cap {
            return;
        }

        let (new_cap, new_buf) = if self.cap == 0 {
            (1, Self::new_buf(1))
        } else {
            let new_cap = self.cap * 2;
            let layout = Layout::array::<T>(new_cap).expect("内存分配错误");
            let new_buf = unsafe {
                let new_buf = realloc(
                    self.data.as_mut() as *mut T as *mut u8,
                    layout,
                    new_cap * std::mem::size_of::<T>(),
                );
                if new_buf.is_null() {
                    panic!("内存分配错误");
                }
                NonNull::new_unchecked(new_buf as *mut _)
            };
            (new_cap, new_buf)
        };

        self.cap = new_cap;
        self.data = new_buf;
    }

    fn new_buf(n: usize) -> NonNull<T> {
        assert_ne!(std::mem::size_of::<T>(), 0, "不支持 ZST");
        let layout = Layout::array::<T>(n).expect("内存分配错误");
        unsafe {
            let buf = alloc(layout);
            if buf.is_null() {
                panic!("内存分配错误");
            }
            NonNull::new_unchecked(buf as *mut _)
        }
    }
}

impl<T> Deref for vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.data.as_ref(), self.len) }
    }
}

impl<T> DerefMut for vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut(), self.len) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v1: vector<usize> = vector::new();
        let mut v2 = vector::with_capacity(2);

        assert_eq!(v2.len(), 0);
        assert_eq!(v2.cap(), 2);
        assert!(v2.empty());

        v2.push(1);
        v2.push(2);
        v2.push(3);

        assert_eq!(v2.len(), 3);
        assert_eq!(v2.cap(), 4);
        assert_eq!(v2[0], 1);
        assert_eq!(v2[1], 2);
        assert_eq!(v2[2], 3);

        assert_eq!(v2.pop(), Some(3));
        assert_eq!(v2.len(), 2);

        v2[0] = 3;
        assert_eq!(v2[0], 3);

        assert_eq!(v2[..], [3, 2]);
    }
}
