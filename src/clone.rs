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
use std::ptr;

impl<V: Clone + Copy, const N: usize> Clone for Stack<V, N> {
    /// Clone it.
    fn clone(&self) -> Self {
        let mut s: Self = Self::new();
        s.next = self.next;
        unsafe { ptr::copy::<V>(self.items.as_ptr(), s.items.as_mut_ptr(), s.next) };
        s
    }
}

#[test]
fn stack_can_be_cloned() {
    let mut s: Stack<u8, 16> = Stack::new();
    s.push(42);
    assert_eq!(42, s.clone().pop().unwrap());
}

#[test]
fn full_stack_can_be_cloned() {
    let mut s: Stack<usize, 16> = Stack::new();
    for i in 0..s.capacity() {
        s.push(i);
    }
    assert_eq!(s.capacity() - 1, s.clone().pop().unwrap());
}

#[test]
fn empty_stack_can_be_cloned() {
    let m: Stack<u8, 0> = Stack::new();
    assert!(m.clone().is_empty());
}
