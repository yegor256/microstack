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

#![feature(test)]

extern crate test;
use microstack::Stack;
use test::Bencher;

const CAPACITY: usize = 4096;

macro_rules! eval {
    ($s:expr) => {{
        let cap = $s.capacity();
        for i in 0..cap {
            $s.push(i);
        }
        $s.clone().clear();
        for i in $s.clone().iter() {
            assert!(*i < cap);
        }
        for _ in 0..cap {
            $s.pop();
        }
        $s.clear();
    }};
}

#[bench]
fn stack_push_and_pop(b: &mut Bencher) {
    b.iter(|| {
        let mut s: Stack<usize, CAPACITY> = Stack::new();
        eval!(s);
    });
}

#[bench]
fn vec_push_and_pop(b: &mut Bencher) {
    b.iter(|| {
        let mut s: Vec<usize> = Vec::with_capacity(CAPACITY);
        eval!(s);
    });
}
