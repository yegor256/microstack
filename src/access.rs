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
use std::ops::Index;

impl<V: Copy, const N: usize> Stack<V, N> {
    /// Return a reference to the element at the given position, counted from
    /// the bottom of the stack, or `None` if the index is out of range.
    #[inline]
    #[must_use]
    pub const fn get(&self, idx: usize) -> Option<&V> {
        if idx < self.next {
            Some(&self.items[idx])
        } else {
            None
        }
    }
}

impl<V: Copy, const N: usize> Index<usize> for Stack<V, N> {
    type Output = V;

    /// Index into the stack by position counted from the bottom.
    ///
    /// # Panics
    ///
    /// Panics when the index is out of range.
    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        assert!(
            idx < self.next,
            "Index {idx} is out of range, the stack has {} item(s)",
            self.next
        );
        &self.items[idx]
    }
}

#[test]
fn get_returns_reference() {
    let mut s: Stack<u64, 4> = Stack::new();
    s.push(10);
    s.push(20);
    assert_eq!(Some(&10), s.get(0));
    assert_eq!(Some(&20), s.get(1));
}

#[test]
fn get_returns_none_out_of_range() {
    let mut s: Stack<u64, 4> = Stack::new();
    s.push(7);
    assert_eq!(None, s.get(1));
    assert_eq!(None, s.get(99));
}

#[test]
fn get_on_empty_stack() {
    let s: Stack<u64, 4> = Stack::new();
    assert_eq!(None, s.get(0));
}

#[test]
fn index_returns_reference() {
    let mut s: Stack<u64, 4> = Stack::new();
    s.push(10);
    s.push(20);
    assert_eq!(10, s[0]);
    assert_eq!(20, s[1]);
}

#[test]
#[should_panic]
fn index_panics_out_of_range() {
    let mut s: Stack<u64, 4> = Stack::new();
    s.push(1);
    let _ = s[5];
}
