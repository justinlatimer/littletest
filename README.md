littletest
====

[![Build Status](https://img.shields.io/travis/justinlatimer/littletest.svg)](https://travis-ci.org/justinlatimer/littletest)
[![](https://img.shields.io/crates/v/littletest.svg)](https://crates.io/crates/littletest)

A Rust testing library inspired by Ruby's [minitest](http://docs.seattlerb.org/minitest/).
Useful for when you're porting something from Ruby or need tests generated at runtime.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]

littletest = "0.2"
```

and this to your crate root:

```rust
extern crate littletest;
```

## Usage

Your tests needs to implement ``Runnable + Sync`` - for example:

```rust
use littletest::{Runnable, TestResult}

struct TestCase {}

impl Runnable for TestCase {
    fn run(&self) -> TestResult {
        TestResult::Pass
    }
}
```

Once you have a list of tests, use a test runner to run them.

```rust
use littletest::{Runnable, TestRunner, TestOptions}

fn run(tests: &Vec<Box<Runnable + Sync>>) {
    let runner = TestRunner::new(TestOptions {
        parallelism: Some(4)
    });
    runner.run(tests);
}
```

## License

MIT, See [LICENSE](LICENSE)
