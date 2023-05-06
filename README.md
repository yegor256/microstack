[![cargo](https://github.com/yegor256/microstack/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/microstack/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/microstack.svg)](https://crates.io/crates/microstack)
[![codecov](https://codecov.io/gh/yegor256/microstack/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/microstack)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/microstack)](https://hitsofcode.com/view/github/yegor256/microstack)
![Lines of code](https://img.shields.io/tokei/lines/github/yegor256/microstack)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/microstack/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/microstack)](https://docs.rs/microstack/latest/microstack/)

Is the simplest and the fastest implementation of a stack on stack. 

First, add this to `Cargo.toml`:

```toml
[dependencies]
microstack = "0.0.0"
```

Then, use it like this:

```rust
use microstack::Stack;
let mut s : Stack<&str, 10> = Stack::new(); // allocation on stack
s.push("foo");
s.push("bar");
assert_eq!(2, s.len());
```

Pay attention, here the stack is created with an extra generic argument `10`. This is 
the total size of the stack, which is allocated on stack when `::new()` is called. 

Read [the API documentation](https://docs.rs/microstack/latest/microstack/).

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
