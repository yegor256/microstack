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

use crate::{IntoIter, Stack};

impl<V: Copy, const N: usize> Iterator for IntoIter<V, N> {
    type Item = V;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.next {
            None
        } else {
            let v = unsafe { self.items.add(self.pos).read() };
            self.pos += 1;
            Some(v)
        }
    }
}

impl<'a, V: Copy + 'a, const N: usize> Stack<V, N> {
    /// Into-iterate them.
    #[inline]
    pub const fn into_iter(&self) -> IntoIter<V, N> {
        IntoIter {
            pos: 0,
            next: self.next,
            items: self.items.as_ptr(),
        }
    }
}

#[test]
fn push_and_iterate() {
    let mut p: Stack<u64, 16> = Stack::new();
    p.push(1);
    p.push(2);
    p.push(3);
    let mut sum = 0;
    for x in p.into_iter() {
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
