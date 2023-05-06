[![cargo](https://github.com/yegor256/microstack/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/microstack/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/microstack.svg)](https://crates.io/crates/microstack)
[![codecov](https://codecov.io/gh/yegor256/microstack/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/microstack)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/microstack)](https://hitsofcode.com/view/github/yegor256/microstack)
![Lines of code](https://img.shields.io/tokei/lines/github/yegor256/microstack)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/microstack/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/microstack)](https://docs.rs/microstack/latest/microstack/)

A much faster alternative of [`HashStack`](https://doc.rust-lang.org/std/collections/struct.HashStack.html), 
for very small maps. 
It is also faster than
[FxHashStack](https://github.com/rust-lang/rustc-hash),
[hashbrown](https://github.com/rust-lang/hashbrown),
[ArrayStack](https://github.com/robjtede/tinymap),
[IndexStack](https://crates.io/crates/indexmap),
and _all_ others.
The smaller the map, the higher the performance. 
It was observed that when a map contains more than 20 keys, it may be better to use the standard 
[`HashStack`](https://doc.rust-lang.org/std/collections/struct.HashStack.html), since
the performance of `microstack::Stack` _may_ start to degrade. 
See the [benchmarking results](#benchmark) below.

**WELCOME**: 
Not all functions that you might expect to have in a map are implemented. 
I will appreciate if you contribute by implementing these 
[missing functions](https://github.com/yegor256/microstack/issues).

First, add this to `Cargo.toml`:

```toml
[dependencies]
microstack = "0.0.12"
```

Then, use it like a standard hash map... well, almost:

```rust
use microstack::Stack;
let mut m : Stack<u64, &str, 10> = Stack::new(); // allocation on stack
m.insert(1, "foo");
m.insert(2, "bar");
assert_eq!(2, m.len());
```

Pay attention, here the map is created with an extra generic argument `10`. This is 
the total size of the map, which is allocated on stack when `::new()` is called. 
Unlike `HashStack`, the `Stack` doesn't use heap at all. If more than ten keys will be
added to the map, it will panic.

Read [the API documentation](https://docs.rs/microstack/latest/microstack/). The struct
[`microstack::Stack`](https://docs.rs/microstack/latest/microstack/struct.Stack.html) is designed as closely similar to 
[`std::collections::HashStack`](https://doc.rust-lang.org/std/collections/struct.HashStack.html) as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `microstack::Stack` with
a few other Rust maps, changing the total capacity of the map (horizontal axis).
We applied the same interactions 
([`benchmark.rs`](https://github.com/yegor256/microstack/blob/master/tests/benchmark.rs)) 
to them and measured how fast they performed. In the following table, 
the numbers over 1.0 indicate performance gain, 
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 2 | 4 | 8 | 16 | 32 | 64 | 128 |
| --- | --: | --: | --: | --: | --: | --: | --: |
| `hashbrown::HashStack` | 26.92 | 8.40 | 3.82 | 2.07 | 0.83 | 0.43 | 0.21 |
| `indexmap::IndexStack` | 19.76 | 12.05 | 5.83 | 3.23 | 1.48 | 0.78 | 0.39 |
| `linear_map::LinearStack` | 3.32 | 1.20 | 0.87 | 0.88 | 0.75 | 0.70 | 0.92 |
| `linked_hash_map::LinkedHashStack` | 32.82 | 17.14 | 7.36 | 4.34 | 2.27 | 1.11 | 0.55 |
| `litemap::LiteStack` | 5.77 | 2.34 | 1.37 | 1.02 | 0.61 | 0.42 | 0.27 |
| `microstack::Stack` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 18.36 | 7.94 | 4.44 | 1.78 | 0.78 | 0.42 | 0.20 |
| `rustc_hash::FxHashStack` | 18.74 | 7.55 | 3.73 | 2.25 | 0.78 | 0.39 | 0.19 |
| `std::collections::BTreeStack` | 27.51 | 8.43 | 4.25 | 3.31 | 1.72 | 0.76 | 0.44 |
| `std::collections::HashStack` | 29.21 | 11.99 | 5.82 | 3.50 | 1.61 | 0.90 | 0.42 |
| `tinymap::array_map::ArrayStack` | 1.66 | 3.54 | 2.89 | 2.94 | 2.95 | 3.56 | 3.53 |

The experiment [was performed](https://github.com/yegor256/microstack/actions/workflows/benchmark.yml) on 30-04-2023.
There were 1000000 repetition cycles.
The entire benchmark took 297s.

<!-- benchmark -->

As you see, the highest performance gain was achieved for the maps that were smaller than ten keys.
For the maps of just a few keys, the gain was enormous.

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
$ cargo test -vv
```

If everything goes well, fork repository, make changes, send us a [pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also, 
run `cargo fmt` and `cargo clippy`.

Also, before you start making changes, run benchmarks:

```bash
$ rustup run nightly cargo bench
```

Then, after the changes you make, run it again. Compare the results. If your changes
degrade performance, think twice before submitting a pull request.
