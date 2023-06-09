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

//! This is a simplest and the fastest implementation of a stack on stack,
//! when stack elements are `Copy` implementing primitives.
//!
//! For example, here is how a stack can be created:
//!
//! ```
//! use microstack::Stack;
//! let mut s : Stack<u64, 10> = Stack::new();
//! s.push(1);
//! s.push(2);
//! assert_eq!(2, s.pop());
//! assert_eq!(1, s.len());
//! ```
//!
//! Creating a [`Stack`] requires knowing the maximum size of it, upfront. This is
//! what the second type argument `10` is for, in the example above. The stack
//! will have exactly ten elements. An attempt to add an 11th element will lead
//! to a panic.

#![doc(html_root_url = "https://docs.rs/microstack/0.0.0")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::multiple_crate_versions)]

use std::marker::PhantomData;

mod clone;
mod ctors;
mod debug;
mod iterators;
#[cfg(feature = "serde")]
mod serialization;
mod stack;

/// This is a simplest and the fastest implementation of a stack on stack,
/// when stack elements are `Copy` implementing primitives.
///
/// For example, here is how a stack can be created:
///
/// ```
/// use microstack::Stack;
/// let mut s : Stack<u64, 10> = Stack::new();
/// s.push(1);
/// s.push(2);
/// assert_eq!(2, s.pop());
/// ```
///
pub struct Stack<V: Copy, const N: usize> {
    /// The next available position in the array.
    next: usize,
    /// The fixed-size array of values.
    items: [V; N],
}

/// Iterator.
pub struct Iter<'a, V: Copy, const N: usize> {
    /// The position.
    pos: usize,
    /// The next available position in the array.
    next: usize,
    /// The fixed-size array of values.
    items: *const V,
    _marker: PhantomData<&'a V>,
}

/// Into-iterator.
pub struct IntoIter<V: Copy, const N: usize> {
    /// The position.
    pos: usize,
    /// The next available position in the array.
    next: usize,
    /// The fixed-size array of values.
    items: *const V,
}
