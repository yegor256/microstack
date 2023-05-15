// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::Stack;

impl<V: Copy, const N: usize> Stack<V, N> {
    /// Make it from vector.
    #[inline]
    #[must_use]
    pub fn from_vec(v: Vec<V>) -> Self {
        let mut p = Self::new();
        for i in v {
            p.push(i);
        }
        p
    }

    /// Get the capacity.
    #[inline]
    #[must_use]
    pub fn capacity(&mut self) -> usize {
        N
    }

    /// Push new element into it.
    #[inline]
    pub fn push(&mut self, v: V) {
        unsafe {
            self.items.as_mut_ptr().add(self.next).write(v);
        }
        self.next += 1;
    }

    /// Push new element into it.
    ///
    /// # Panics
    ///
    /// If there is no more space in the stack, it will panic.
    #[inline]
    pub fn try_push(&mut self, v: V) {
        assert!(self.next < N, "No more space left in the stack");
        self.push(v);
    }

    /// Pop a element from it.
    #[inline]
    pub fn pop(&mut self) -> V {
        self.next -= 1;
        unsafe { self.items.as_ptr().add(self.next).read() }
    }

    /// Pop a element from it.
    ///
    /// If there is no more elements left, it will return `None`.
    #[inline]
    #[must_use]
    pub fn try_pop(&mut self) -> Option<V> {
        if self.next == 0 {
            None
        } else {
            Some(self.pop())
        }
    }

    /// Clear.
    #[inline]
    pub fn clear(&mut self) {
        self.next = 0;
    }

    /// Is it empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Length of it.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.next
    }
}

#[test]
fn push_one() {
    let mut s: Stack<u64, 1> = Stack::new();
    s.push(42);
    assert_eq!(42, s.pop());
}

#[test]
fn push_safely() {
    let mut s: Stack<u64, 1> = Stack::new();
    s.try_push(42);
    assert_eq!(42, s.pop());
}

#[test]
fn build_from_vec() {
    let mut s: Stack<u64, 1> = Stack::from_vec(vec![42]);
    assert_eq!(42, s.pop());
}

#[test]
fn pop_none() {
    let mut s: Stack<u64, 1> = Stack::new();
    assert_eq!(0, s.len());
    assert!(s.is_empty());
    assert!(s.try_pop().is_none());
}

#[test]
fn with_str() {
    let mut s: Stack<&str, 1> = Stack::new();
    s.push("Hello!");
    assert_eq!("Hello!", s.pop());
}

#[test]
#[should_panic]
fn panic_on_empty_stack() {
    let mut s: Stack<u64, 0> = Stack::new();
    assert_eq!(0, s.len());
    s.try_push(11);
}

#[test]
fn push_and_pop() {
    let mut s: Stack<u64, 16> = Stack::new();
    s.push(42);
    assert_eq!(42, s.pop());
}
