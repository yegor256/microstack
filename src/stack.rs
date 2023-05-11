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

impl<V : Copy, const N: usize> Stack<V, N> {
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

    /// Pop a element from it.
    #[inline]
    pub fn pop(&mut self) -> Option<V> {
        if self.next == 0 {
            None
        } else {
            self.next -= 1;
            Some(unsafe { self.items.as_ptr().add(self.next).read() })
        }
    }

    /// Clear.
    #[inline]
    pub fn clear(&mut self) {
        self.next = 0;
    }

    /// Is it empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Length of it.
    #[inline]
    pub const fn len(&self) -> usize {
        self.next
    }

    /// Iterate them.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &V> {
        self.items.iter().take(self.next)
    }
}

impl<'a, V: Clone + Copy + 'a, const N: usize> Stack<V, N> {
    /// Into-iterate them.
    #[inline]
    pub fn into_iter(&self) -> impl Iterator<Item = V> + '_ {
        self.items.iter().take(self.next).cloned()
    }
}

#[test]
fn push_one() {
    let mut s: Stack<u64, 1> = Stack::new();
    s.push(42);
    assert_eq!(42, s.pop().unwrap());
}

#[test]
fn pop_none() {
    let mut s: Stack<u64, 1> = Stack::new();
    assert_eq!(0, s.len());
    assert!(s.is_empty());
    assert!(s.pop().is_none());
}

#[test]
fn push_and_pop() {
    let mut s: Stack<u64, 16> = Stack::new();
    s.push(42);
    assert_eq!(42, s.pop().unwrap());
}

#[test]
fn push_and_iterate() {
    let mut p: Stack<u64, 16> = Stack::new();
    p.push(1);
    p.push(2);
    p.push(3);
    let mut sum = 0;
    for x in p.iter() {
        sum += x;
    }
    assert_eq!(6, sum);
}

#[test]
fn push_and_into_iterate() {
    let mut p: Stack<u64, 16> = Stack::new();
    p.push(1);
    p.push(2);
    let mut sum = 0;
    for x in p.into_iter() {
        sum += x;
    }
    assert_eq!(3, sum);
}

#[test]
fn push_clear_and_iterate() {
    let mut p: Stack<u64, 16> = Stack::new();
    p.push(1);
    p.push(2);
    p.push(3);
    assert_eq!(3, p.len());
    p.clear();
    assert_eq!(0, p.len());
    let mut sum = 0;
    for x in p.iter() {
        sum += x;
    }
    assert_eq!(0, sum);
}
