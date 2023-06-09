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
use std::mem::MaybeUninit;

impl<V: Copy, const N: usize> Default for Stack<V, N> {
    /// Make a default empty [`Stack`].
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Copy, const N: usize> Stack<V, N> {
    /// Make it.
    ///
    /// The size of the stack is defined by the generic argument.
    #[inline]
    #[must_use]
    #[allow(clippy::uninit_assumed_init)]
    pub const fn new() -> Self {
        unsafe {
            Self {
                next: 0,
                items: MaybeUninit::<[V; N]>::uninit().assume_init(),
            }
        }
    }
}

#[test]
fn makes_default_stack() {
    let s: Stack<u8, 8> = Stack::default();
    assert_eq!(0, s.len());
}

#[test]
fn makes_new_stack() {
    let s: Stack<u8, 8> = Stack::new();
    assert_eq!(0, s.len());
}
