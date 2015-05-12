extern crate littletest;

use littletest::{Runnable, TestResult, TestOptions, TestRunner};

struct TestCase {
    result: TestResult
}

impl TestCase {
    fn new(result: TestResult) -> TestCase {
        TestCase {
            result: result
        }
    }
}

impl Runnable for TestCase {
    fn run(&self) -> TestResult {
        self.result.clone()
    }
}

#[test]
fn it_works() {
    use std::iter::repeat;

    let runnables = repeat(TestResult::Pass)
        .take(10)
        .map(|result| Box::new(TestCase::new(result)) as Box<Runnable + Sync>)
        .collect::<Vec<_>>();

    let runner = TestRunner::new(TestOptions {
        parallelism: Some(4)
    });
    runner.run(&runnables);
}
